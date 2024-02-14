//! Defines the [`Storable`] trait, which defines the `.load` and `.store` functions
//! that are implemented for most WNFS structures, such as `PublicFile`, `PublicDirectory`,
//! `PublicNode`, `HamtForest` etc.
use crate::{
    utils::{Arc, CondSend, CondSync},
    BlockStore,
};
use anyhow::{bail, Result};
use async_once_cell::OnceCell;
use bytes::Bytes;
use futures::Future;
use libipld::{cbor::DagCborCodec, Cid};
use serde::{de::DeserializeOwned, Serialize};

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

#[macro_export]
macro_rules! impl_storable_from_serde {
    ( $( $ty:ty $( : < $( $generics:ident ),+ > )? ),+ ) => {
        $(
            impl $( < $( $generics ),+ > )? $crate::Storable for $ty $( where $( $generics: ::serde::Serialize + ::serde::de::DeserializeOwned + Clone + $crate::utils::CondSync ),+  )?{
                type Serializable = $ty;

                async fn to_serializable(&self, _store: &impl $crate::BlockStore) -> ::anyhow::Result<Self::Serializable> {
                    Ok(self.clone())
                }

                async fn from_serializable(_cid: Option<&$crate::libipld::Cid>, serializable: Self::Serializable) -> ::anyhow::Result<Self> {
                    Ok(serializable)
                }
            }
        )+
    };
}

pub use impl_storable_from_serde;

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// The trait that defines how to store something in a blockstore.
///
/// This works via a two-tiered system, where the actual in-memory representation
/// (the struct that implements this trait) is not the same as the at-rest
/// representation of itself.
/// The at-rest representation is given by the `Serializable` associated type.
///
/// Commonly, the `Serializable` type implements serde's `Serialize` and `Deserialize`
/// traits and thus can automatically be used without having to implement `StoreIpld`
/// and `LoadIpld` yourself. In that case, the default implementation will use
/// `serde_ipld_dagcbor`.
///
/// This trait also optionally supports memoizing serialization via the `persisted_as` function.
/// You can add a field `persisted_as: OnceCell<Cid>` to your in-memory representation and
/// return it in the `persisted_as` function and any `store` calls will automatically populate
/// that cache.
/// If you do so, remember to initialize the `OnceCell` if a `Cid` is passed in the
/// `from_serializable` call, such that a `store` call right after a `load` call is practically
/// free.
pub trait Storable: Sized {
    /// The at-rest representation of this storable type.
    type Serializable: StoreIpld + LoadIpld + CondSync;

    /// Turn the current type into the at-rest representation of this type.
    fn to_serializable(
        &self,
        store: &impl BlockStore,
    ) -> impl Future<Output = Result<Self::Serializable>> + CondSend;

    /// Take an at-rest representation of this type and turn it into the in-memory representation.
    /// You can use the `cid` parameter to populate a cache.
    fn from_serializable(
        cid: Option<&Cid>,
        serializable: Self::Serializable,
    ) -> impl Future<Output = Result<Self>> + CondSend;

    /// Return a serialization cache, if it exists.
    /// By default, this always returns `None`.
    fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
        None
    }

    /// Store this data type in a given `BlockStore`.
    ///
    /// This will short-circuit by using the `persisted_as` once-cell, if available.
    fn store(&self, store: &impl BlockStore) -> impl Future<Output = Result<Cid>> + CondSend
    where
        Self: CondSync,
    {
        let store_future = async {
            let (bytes, codec) = self.to_serializable(store).await?.encode_ipld()?;
            store.put_block(bytes, codec).await
        };

        async {
            if let Some(persisted_as) = self.persisted_as() {
                persisted_as.get_or_try_init(store_future).await.cloned()
            } else {
                store_future.await
            }
        }
    }

    /// Try to load a value of this type from a CID.
    ///
    /// This will pass on the CID to the `from_serializable` function so it can
    /// populate a cache in some cases.
    fn load(cid: &Cid, store: &impl BlockStore) -> impl Future<Output = Result<Self>> + CondSend {
        async {
            let bytes = store.get_block(cid).await?;
            let serializable = Self::Serializable::decode_ipld(cid, bytes)?;
            Self::from_serializable(Some(cid), serializable).await
        }
    }
}

pub trait StoreIpld {
    fn encode_ipld(&self) -> Result<(Bytes, u64)>;
}

pub trait LoadIpld: Sized {
    fn decode_ipld(cid: &Cid, bytes: Bytes) -> Result<Self>;
}

impl<T: Serialize> StoreIpld for T {
    fn encode_ipld(&self) -> Result<(Bytes, u64)> {
        let bytes = serde_ipld_dagcbor::to_vec(self)?;
        Ok((bytes.into(), DagCborCodec.into()))
    }
}

impl<T: DeserializeOwned + Sized> LoadIpld for T {
    fn decode_ipld(cid: &Cid, bytes: Bytes) -> Result<Self> {
        let codec = cid.codec();
        let dag_cbor: u64 = DagCborCodec.into();
        if codec != dag_cbor {
            bail!("Expected dag-cbor codec, but got {codec:X} in CID {cid}");
        }
        Ok(serde_ipld_dagcbor::from_slice(bytes.as_ref())?)
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

// We need to choose *one* blanket implementation, and unfortunately
// you can't `impl Storable for Arc<MyType>` outside of this module,
// because that'd be an orphan instance. So instead we're providing a
// macro and implement the `Arc<T>` instance generically here.

// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// impl<T: StoreIpld + LoadIpld + CondSync + Clone> Storable for T {
//     type Serializable = T;

//     async fn to_serializable(&self, _store: &impl BlockStore) -> Result<Self::Serializable> {
//         Ok(self.clone())
//     }

//     async fn from_serializable(serializable: Self::Serializable) -> Result<Self> {
//         Ok(serializable)
//     }
// }

impl<T: Storable + CondSync> Storable for Arc<T> {
    type Serializable = T::Serializable;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
        self.as_ref().to_serializable(store).await
    }

    async fn from_serializable(
        cid: Option<&Cid>,
        serializable: Self::Serializable,
    ) -> Result<Self> {
        Ok(Arc::new(T::from_serializable(cid, serializable).await?))
    }

    fn persisted_as(&self) -> Option<&OnceCell<Cid>> {
        self.as_ref().persisted_as()
    }
}

impl_storable_from_serde! { [u8; 0], [u8; 1], [u8; 2], [u8; 4], [u8; 8], [u8; 16], [u8; 32] }
impl_storable_from_serde! { usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8 }
impl_storable_from_serde! { String }
impl_storable_from_serde! {
    (A,): <A>,
    (A, B): <A, B>,
    (A, B, C): <A, B, C>,
    (A, B, C, D): <A, B, C, D>,
    (A, B, C, D, E): <A, B, C, D, E>,
    (A, B, C, D, E, F): <A, B, C, D, E, F>,
    (A, B, C, D, E, F, G): <A, B, C, D, E, F, G>,
    (A, B, C, D, E, F, G, H): <A, B, C, D, E, F, G, H>,
    (A, B, C, D, E, F, G, H, I): <A, B, C, D, E, F, G, H, I>,
    (A, B, C, D, E, F, G, H, I, J): <A, B, C, D, E, F, G, H, I, J>,
    (A, B, C, D, E, F, G, H, I, J, K): <A, B, C, D, E, F, G, H, I, J, K>
}
