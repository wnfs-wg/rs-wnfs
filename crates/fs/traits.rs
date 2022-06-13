use crate::BlockStore;

use anyhow::Result;
use async_trait::async_trait;
use libipld::{error::SerdeError, serde as ipld_serde, Ipld};
use serde::{Serialize, Serializer};

//--------------------------------------------------------------------------------------------------
// Traits
//--------------------------------------------------------------------------------------------------

/// Implements getting a unique identifier for a node.
pub trait Id {
    /// Gets an identifier for the node.
    fn get_id(&self) -> String;
}

/// Implements deep equality check for two types.
#[async_trait(?Send)]
pub trait DeepEq {
    /// Checks if the two items are deeply equal.
    async fn deep_eq<B: BlockStore>(&self, other: &Self, store: &mut B) -> Result<bool>;
}

/// A **data structure** that can be serialized into any data format supported
/// by Serde.
///
/// This trait is slightly different from Serde's Serialize trait because it allows for asynchronous
/// serialisation and it is designed for the IPLD ecosystem where a `BlockStore` is sometimes needed to
/// properly resolve the internal state of certain data structures to Cids.
///
/// An example of this is the PublicDirectory which can contain links to other IPLD nodes.
/// These links need to be resolved to Cids during serialization if they aren't already.
#[async_trait(?Send)]
pub trait AsyncSerialize {
    /// Serializes the type.
    async fn async_serialize<S: Serializer, B: BlockStore + ?Sized>(
        &self,
        serializer: S,
        store: &mut B,
    ) -> Result<S::Ok, S::Error>;

    /// Serialize with an IPLD serializer.
    async fn async_serialize_ipld<B: BlockStore + ?Sized>(
        &self,
        store: &mut B,
    ) -> Result<Ipld, SerdeError> {
        self.async_serialize(ipld_serde::Serializer, store).await
    }
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

/// Implements async deserialization for serde serializable types.
#[async_trait(?Send)]
impl<T: Serialize> AsyncSerialize for T {
    async fn async_serialize<S: Serializer, B: BlockStore + ?Sized>(
        &self,
        serializer: S,
        _: &mut B,
    ) -> Result<S::Ok, S::Error> {
        self.serialize(serializer)
    }
}
