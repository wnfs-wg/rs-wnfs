use super::{
    forest::traits::PrivateForest, PrivateDirectory, PrivateFile, PrivateNode, PrivateRef,
};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use rand_core::CryptoRngCore;
use std::rc::Rc;
use wnfs_common::BlockStore;
use wnfs_nameaccumulator::{AccumulatorSetup, Name};

#[derive(Debug)]
pub(crate) enum PrivateLink {
    Encrypted {
        private_ref: PrivateRef,
        cache: OnceCell<PrivateNode>,
    },
    Decrypted {
        // In this case, the `PrivateNode` contains its own `OnceCell<Cid>`
        // which if full, combined with the `PrivateNode` derives the `PrivateRef`.
        node: PrivateNode,
    },
}

impl PrivateLink {
    pub(crate) fn from_ref(private_ref: PrivateRef) -> Self {
        Self::Encrypted {
            private_ref,
            cache: OnceCell::new(),
        }
    }

    #[async_recursion(?Send)]
    pub(crate) async fn resolve_ref(
        &self,
        forest: &mut impl PrivateForest,
        store: &impl BlockStore,
        rng: &mut impl CryptoRngCore,
    ) -> Result<PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Ok(private_ref.clone()),
            Self::Decrypted { node } => Ok(node.store(forest, store, rng).await?),
        }
    }

    pub(crate) async fn resolve_node(
        &self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
        parent_name: Option<Name>,
    ) -> Result<&PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => {
                cache
                    .get_or_try_init(PrivateNode::load(private_ref, forest, store, parent_name))
                    .await
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    /// Gets mut value stored in link. It attempts to get it from the store if it is not present in link.
    pub(crate) async fn resolve_node_mut(
        &mut self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
        parent_name: Option<Name>,
    ) -> Result<&mut PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => {
                let private_node = match cache.take() {
                    Some(node) => node,
                    None => PrivateNode::load(private_ref, forest, store, parent_name).await?,
                };

                // We need to switch this PrivateLink to be a `Decrypted` again, since
                // mutations on the `PrivateNode` may change the `private_ref`, e.g. by
                // advancing the ratchet forward.
                // So the `PrivateRef` should be managed by the `PrivateNode` itself
                // rather than the `PrivateLink`.
                *self = Self::Decrypted { node: private_node };

                Ok(match self {
                    Self::Decrypted { node, .. } => node,
                    _ => unreachable!(),
                })
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    /// Gets an owned value from type. It attempts to it get from the store if it is not present in type.
    pub(crate) async fn resolve_owned_node(
        self,
        forest: &impl PrivateForest,
        store: &impl BlockStore,
        parent_name: Option<Name>,
    ) -> Result<PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => {
                    let node = PrivateNode::load(&private_ref, forest, store, parent_name).await?;
                    node.persisted_as()
                        .get_or_init(async { private_ref.content_cid })
                        .await;
                    Ok(node)
                }
            },
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    /// Creates a link to a directory node.
    #[inline]
    pub(crate) fn with_dir(dir: PrivateDirectory) -> Self {
        Self::from(PrivateNode::Dir(Rc::new(dir)))
    }

    /// Creates a link to a file node.
    #[inline]
    pub(crate) fn with_file(file: PrivateFile) -> Self {
        Self::from(PrivateNode::File(Rc::new(file)))
    }

    #[allow(dead_code)]
    pub(crate) fn get_ref(&self, setup: &AccumulatorSetup) -> Option<PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Some(private_ref.clone()),
            Self::Decrypted { node } => node.get_private_ref(setup),
        }
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
            (Self::Encrypted { private_ref, cache }, Self::Decrypted { node }) => {
                Some(&private_ref.content_cid) == node.persisted_as().get()
                    || Some(node) == cache.get()
            }
            (Self::Decrypted { node }, Self::Encrypted { private_ref, cache }) => {
                Some(&private_ref.content_cid) == node.persisted_as().get()
                    || Some(node) == cache.get()
            }
        }
    }
}

impl Clone for PrivateLink {
    fn clone(&self) -> Self {
        match self {
            Self::Encrypted { private_ref, cache } => Self::Encrypted {
                private_ref: private_ref.clone(),
                cache: OnceCell::new_with(cache.get().cloned()),
            },
            Self::Decrypted { node } => Self::Decrypted { node: node.clone() },
        }
    }
}

impl From<PrivateNode> for PrivateLink {
    fn from(node: PrivateNode) -> Self {
        Self::Decrypted { node }
    }
}
