use crate::{
    utils::{Arc, CondSend, CondSync},
    BlockStore,
};
use anyhow::{bail, Result};
use async_trait::async_trait;
use bytes::Bytes;
use libipld::{cbor::DagCborCodec, error::SerdeError, serde as ipld_serde, Cid, Ipld};
use serde::{de::DeserializeOwned, Serialize, Serializer};

//--------------------------------------------------------------------------------------------------
// Macros
//--------------------------------------------------------------------------------------------------

macro_rules! impl_async_serialize {
    ( $( $ty:ty $( : < $( $generics:ident ),+ > )? ),+ ) => {
        $(
            #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
            #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
            impl $( < $( $generics ),+ > )? AsyncSerialize for $ty $( where $( $generics: Serialize + CondSync ),+  )? {
                async fn async_serialize<S: Serializer + CondSend, BS: BlockStore>(
                    &self,
                    serializer: S,
                    _: &BS,
                ) -> Result<S::Ok, S::Error> {
                    self.serialize(serializer)
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_storable_from_serde {
    ( $( $ty:ty $( : < $( $generics:ident ),+ > )? ),+ ) => {
        $(
            #[cfg_attr(not(target_arch = "wasm32"), ::async_trait::async_trait)]
            #[cfg_attr(target_arch = "wasm32", ::async_trait::async_trait(?Send))]
            impl $( < $( $generics ),+ > )? $crate::Storable for $ty $( where $( $generics: ::serde::Serialize + ::serde::de::DeserializeOwned + Clone + $crate::utils::CondSync ),+  )?{
                type Serializable = $ty;

                async fn to_serializable(&self, _store: &impl $crate::BlockStore) -> ::anyhow::Result<Self::Serializable> {
                    Ok(self.clone())
                }

                async fn from_serializable(serializable: Self::Serializable) -> ::anyhow::Result<Self> {
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

/// A **data structure** that can be serialized into any data format supported
/// by Serde.
///
/// This trait is slightly different from Serde's Serialize trait because it allows for asynchronous
/// serialisation and it is designed for the IPLD ecosystem where a `Store` is sometimes needed to
/// properly resolve the internal state of certain data structures to Cids.
///
/// An example of this is the PublicDirectory which can contain links to other IPLD nodes.
/// These links need to be resolved to Cids during serialization if they aren't already.
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait AsyncSerialize {
    /// Serializes the type.
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer + CondSend,
        S::Error: CondSend,
        B: BlockStore + ?Sized;

    /// Serialize with an IPLD serializer.
    async fn async_serialize_ipld<B>(&self, store: &B) -> Result<Ipld, SerdeError>
    where
        B: BlockStore + ?Sized,
    {
        self.async_serialize(ipld_serde::Serializer, store).await
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub trait Storable: Sized {
    type Serializable: StoreIpld + LoadIpld + CondSync;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable>;

    async fn from_serializable(serializable: Self::Serializable) -> Result<Self>;

    async fn store(&self, store: &impl BlockStore) -> Result<Cid> {
        let (bytes, codec) = self.to_serializable(store).await?.encode_ipld()?;
        store.put_block(bytes, codec).await
    }

    async fn load(cid: &Cid, store: &impl BlockStore) -> Result<Self> {
        let bytes = store.get_block(cid).await?;
        let serializable = Self::Serializable::decode_ipld(cid, bytes)?;
        Self::from_serializable(serializable).await
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

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T: Storable + CondSync> Storable for Arc<T> {
    type Serializable = T::Serializable;

    async fn to_serializable(&self, store: &impl BlockStore) -> Result<Self::Serializable> {
        self.as_ref().to_serializable(store).await
    }

    async fn from_serializable(serializable: Self::Serializable) -> Result<Self> {
        Ok(Arc::new(T::from_serializable(serializable).await?))
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
impl<T: AsyncSerialize + CondSync> AsyncSerialize for Arc<T> {
    async fn async_serialize<S, B>(&self, serializer: S, store: &B) -> Result<S::Ok, S::Error>
    where
        S: Serializer + CondSend,
        S::Error: CondSend,
        B: BlockStore + ?Sized,
    {
        self.as_ref().async_serialize(serializer, store).await
    }
}

impl_async_serialize! { usize, u128, u64, u32, u16, u8, isize, i128, i64, i32, i16, i8 }
impl_async_serialize! { String, &str }
impl_async_serialize! {
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
