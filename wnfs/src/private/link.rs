use super::{encrypted::Encrypted, PrivateForest, PrivateRef};
use crate::{BlockStore, PrivateNode};
use anyhow::Result;
use async_once_cell::OnceCell;
use rand_core::RngCore;
use std::rc::Rc;

#[derive(Debug)]
pub enum PrivateLink {
    Encrypted {
        private_ref: PrivateRef,
        node_cache: OnceCell<PrivateNode>,
    },
    Decrypted {
        node: PrivateNode,
        private_ref_cache: OnceCell<PrivateRef>,
    },
}

impl PrivateLink {
    pub fn from_ref(private_ref: PrivateRef) -> Self {
        Self::Encrypted {
            private_ref,
            node_cache: OnceCell::new(),
        }
    }

    pub fn new(node: PrivateNode) -> Self {
        Self::Decrypted {
            node,
            private_ref_cache: OnceCell::new(),
        }
    }

    pub async fn resolve_ref(
        &self,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<&PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Ok(private_ref),
            Self::Decrypted {
                node,
                private_ref_cache,
            } => {
                private_ref_cache
                    .get_or_try_init(async { forest.put(node, store, rng).await })
                    .await
            }
        }
    }

    pub async fn resolve_node(
        &self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<&PrivateNode> {
        match self {
            Self::Encrypted {
                private_ref,
                node_cache,
            } => {
                node_cache
                    .get_or_try_init(async { forest.get(private_ref, store).await })
                    .await
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    pub fn get_ref(&self) -> Option<&PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Some(private_ref),
            Self::Decrypted {
                private_ref_cache, ..
            } => private_ref_cache.get(),
        }
    }

    //-----------------------------------
    // WNFS specific stuff.
    // Consider breaking this out once we
    // have something like PrivateLink<T>
    //-----------------------------------

    /// This should be called to prepare a node for modifications,
    /// if it's meant to be a successor revision of the current revision.
    ///
    /// It will store the current revision in the given `BlockStore` to
    /// retrieve its CID and put that into the `previous` links,
    /// as well as advancing the ratchet and resetting the `persisted_as` pointer.
    pub async fn prepare_next_revision(
        &mut self,
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<()> {
        let Some(_) = self.get_ref() else {
            // If there's no PrivateRef associated with it
            // we can skip preparing, since we're already
            // in a sort-of "staging" area.
            return Ok(());
        };
        let node = self.resolve_node(forest, store).await?.clone();
        // key from the *current*, not the next revision
        let temporal_key = node.get_header().derive_temporal_key();

        let header_cid = node.get_header().store(store).await?;

        let node = match node {
            PrivateNode::File(mut file_rc) => {
                let file = Rc::make_mut(&mut file_rc);

                let snapshot_key = temporal_key.derive_snapshot_key();

                let content_cid = file
                    .content
                    .store(header_cid, &snapshot_key, store, rng)
                    .await?;

                file.content.previous.clear();
                file.content
                    .previous
                    .insert((1, Encrypted::from_value(content_cid, &temporal_key)?));

                file.header.advance_ratchet();

                PrivateNode::File(file_rc)
            }
            PrivateNode::Dir(mut dir_rc) => {
                let dir = Rc::make_mut(&mut dir_rc);

                let content_cid = dir
                    .content
                    .store(header_cid, &temporal_key, forest, store, rng)
                    .await?;

                dir.content.previous.clear();
                dir.content
                    .previous
                    .insert((1, Encrypted::from_value(content_cid, &temporal_key)?));

                dir.header.advance_ratchet();

                PrivateNode::Dir(dir_rc)
            }
        };

        // We make sure to clear any cached PrivateRefs
        *self = Self::new(node);

        Ok(())
    }
}

impl PartialEq for PrivateLink {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Encrypted {
                    private_ref: l_private_ref,
                    ..
                },
                Self::Encrypted {
                    private_ref: r_private_ref,
                    ..
                },
            ) => l_private_ref == r_private_ref,
            (Self::Decrypted { node: l_node, .. }, Self::Decrypted { node: r_node, .. }) => {
                l_node == r_node
            }
            (
                Self::Encrypted {
                    private_ref,
                    node_cache,
                },
                Self::Decrypted {
                    private_ref_cache,
                    node,
                },
            ) => Some(private_ref) == private_ref_cache.get() || Some(node) == node_cache.get(),
            (
                Self::Decrypted {
                    private_ref_cache,
                    node,
                },
                Self::Encrypted {
                    private_ref,
                    node_cache,
                },
            ) => Some(private_ref) == private_ref_cache.get() || Some(node) == node_cache.get(),
        }
    }
}

impl Clone for PrivateLink {
    fn clone(&self) -> Self {
        match self {
            Self::Encrypted {
                private_ref,
                node_cache,
            } => Self::Encrypted {
                private_ref: private_ref.clone(),
                node_cache: OnceCell::new_with(node_cache.get().cloned()),
            },
            Self::Decrypted {
                node,
                private_ref_cache,
            } => Self::Decrypted {
                node: node.clone(),
                private_ref_cache: OnceCell::new_with(private_ref_cache.get().cloned()),
            },
        }
    }
}
