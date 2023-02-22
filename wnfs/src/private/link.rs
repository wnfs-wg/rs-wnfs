use super::{PrivateForest, PrivateRef};
use crate::{BlockStore, PrivateNode};
use anyhow::Result;
use async_once_cell::OnceCell;
use async_recursion::async_recursion;
use rand_core::RngCore;
use std::rc::Rc;

#[derive(Debug)]
pub enum PrivateLink {
    Encrypted {
        private_ref: PrivateRef,
        cache: OnceCell<PrivateNode>,
    },
    Decrypted {
        node: PrivateNode,
    },
}

impl PrivateLink {
    pub fn from_ref(private_ref: PrivateRef) -> Self {
        Self::Encrypted {
            private_ref,
            cache: OnceCell::new(),
        }
    }

    pub fn new(node: PrivateNode) -> Self {
        Self::Decrypted { node }
    }

    #[async_recursion(?Send)]
    pub async fn resolve_ref(
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

    pub async fn resolve_node(
        &self,
        forest: &PrivateForest,
        store: &impl BlockStore,
    ) -> Result<&PrivateNode> {
        match self {
            Self::Encrypted { private_ref, cache } => {
                cache
                    .get_or_try_init(async { forest.get(private_ref, store).await })
                    .await
            }
            Self::Decrypted { node, .. } => Ok(node),
        }
    }

    pub fn get_ref(&self) -> Option<PrivateRef> {
        match self {
            Self::Encrypted { private_ref, .. } => Some(private_ref.clone()),
            Self::Decrypted { node } => node.get_ref_if_stored(),
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
                Some(private_ref) == node.get_ref_if_stored().as_ref() || Some(node) == cache.get()
            }
            (Self::Decrypted { node }, Self::Encrypted { private_ref, cache }) => {
                Some(private_ref) == node.get_ref_if_stored().as_ref() || Some(node) == cache.get()
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