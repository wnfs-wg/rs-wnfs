use async_once_cell::OnceCell;

use super::PrivateRef;

#[derive(Debug)]
pub enum PrivateLink<T> {
    Resolved {
        resolved: T,
        reference_cache: OnceCell<PrivateRef>,
    },
    Unresolved {
        resolved_cache: OnceCell<T>,
        reference: PrivateRef,
    },
}

impl<T> PrivateLink<T> {
    fn from_ref(reference: PrivateRef) -> Self {
        Self::Unresolved {
            reference,
            resolved_cache: OnceCell::new(),
        }
    }

    fn get_ref(&self) -> Option<&PrivateRef> {
        match self {
            PrivateLink::Resolved {
                reference_cache, ..
            } => reference_cache.get(),
            PrivateLink::Unresolved { reference, .. } => Some(reference),
        }
    }

    fn get_value(&self) -> Option<&T> {
        match self {
            PrivateLink::Resolved { resolved, .. } => Some(resolved),
            PrivateLink::Unresolved { resolved_cache, .. } => resolved_cache.get(),
        }
    }
}

impl<T: Clone> Clone for PrivateLink<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Resolved {
                resolved,
                reference_cache,
            } => Self::Resolved {
                resolved: resolved.clone(),
                reference_cache: OnceCell::new_with(reference_cache.get().cloned()),
            },
            Self::Unresolved {
                resolved_cache,
                reference,
            } => Self::Unresolved {
                reference: reference.clone(),
                resolved_cache: OnceCell::new_with(resolved_cache.get().cloned()),
            },
        }
    }
}

impl<T: PartialEq> PartialEq for PrivateLink<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Resolved {
                    resolved: l_resolved,
                    ..
                },
                Self::Resolved {
                    resolved: r_resolved,
                    ..
                },
            ) => l_resolved == r_resolved,
            (
                Self::Unresolved {
                    reference: l_reference,
                    ..
                },
                Self::Unresolved {
                    reference: r_reference,
                    ..
                },
            ) => l_reference == r_reference,
            (Self::Unresolved { reference, .. }, Self::Resolved { resolved, .. }) => {
                if let Some(reference2) = other.get_ref() {
                    reference == reference2
                }
            }
        }
    }
}
