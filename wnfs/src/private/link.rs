use super::{PrivateDirectory, PrivateFile, PrivateForest, PrivateNode, PrivateRef};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use rand_core::RngCore;
use std::rc::Rc;
use wnfs_common::BlockStore;

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
        forest: &mut Rc<PrivateForest>,
        store: &mut impl BlockStore,
        rng: &mut impl RngCore,
    ) -> Result<PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Ok(private_ref.clone()),
            Self::Decrypted { node } => Ok(node.store(forest, store, rng).await?),
        }
    }

    pub(crate) async fn resolve_node(
        &self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<&PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => {
                cache
                    .get_or_try_init(PrivateNode::load(private_ref, forest, store))
                    .await
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    /// Gets mut value stored in link. It attempts to get it from the store if it is not present in link.
    pub(crate) async fn resolve_node_mut(
        &mut self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<&mut PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => {
                let private_node = match cache.take() {
                    Some(node) => node,
                    None => PrivateNode::load(private_ref, forest, store).await?,
                };

                *self = Self::Decrypted { node: private_node };

                Ok(match self {
                    Self::Encrypted { .. } => unreachable!(),
                    Self::Decrypted { node, .. } => node,
                })
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    /// Gets an owned value from type. It attempts to it get from the store if it is not present in type.
    pub(crate) async fn resolve_owned_node(
        self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => match cache.into_inner() {
                Some(cached) => Ok(cached),
                None => {
                    let node = PrivateNode::load(&private_ref, forest, store).await?;
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
    pub(crate) fn get_ref(&self) -> Option<PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Some(private_ref.clone()),
            Self::Decrypted { node } => node.get_private_ref(),
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
                Some(private_ref) == node.get_private_ref().as_ref() || Some(node) == cache.get()
            }
            (Self::Decrypted { node }, Self::Encrypted { private_ref, cache }) => {
                Some(private_ref) == node.get_private_ref().as_ref() || Some(node) == cache.get()
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
