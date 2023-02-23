//! The bindgen API for WNFS block store.

use super::utils::anyhow_error;
use anyhow::Result;
use async_trait::async_trait;
use js_sys::{Promise, Uint8Array};
use std::borrow::Cow;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_futures::JsFuture;
use wnfs::{
    libipld::{Cid, IpldCodec},
    BlockStore as WnfsBlockStore,
};

//--------------------------------------------------------------------------------------------------
// Externs
//--------------------------------------------------------------------------------------------------

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "BlockStore")]
    pub type BlockStore;

    #[wasm_bindgen(method, js_name = "putBlock")]
    pub(crate) fn put_block(store: &BlockStore, bytes: Vec<u8>, code: Code) -> Promise;

    #[wasm_bindgen(method, js_name = "getBlock")]
    pub(crate) fn get_block(store: &BlockStore, cid: Vec<u8>) -> Promise;
}

//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents the format the content a CID points to.
///
/// The variants are based on the ipld and multiformats specification.
///
/// - https://ipld.io/docs/codecs/#known-codecs
/// - https://github.com/multiformats/multicodec/blob/master/table.csv
#[wasm_bindgen]
pub enum Code {
    DagProtobuf = 0x70,
    DagCbor = 0x71,
    DagJson = 0x0129,
    Raw = 0x55,
}

/// A block store provided by the host (JavaScript) for custom implementation like connection to the IPFS network.
#[wasm_bindgen]
pub struct ForeignBlockStore(pub(crate) BlockStore);

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

#[async_trait(?Send)]
impl WnfsBlockStore for ForeignBlockStore {
    /// Stores an array of bytes in the block store.
    async fn put_block(&mut self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        let value = JsFuture::from(self.0.put_block(bytes, codec.into()))
            .await
            .map_err(anyhow_error("Cannot get block: {:?}"))?;

        // Convert the value to a vector of bytes.
        let bytes = Uint8Array::new(&value).to_vec();

        // Construct CID from the bytes.
        Ok(Cid::try_from(&bytes[..])?)
    }

    /// Retrieves an array of bytes from the block store with given CID.
    async fn get_block<'a>(&'a self, cid: &Cid) -> Result<Cow<'a, Vec<u8>>> {
        let value = JsFuture::from(self.0.get_block(cid.to_bytes()))
            .await
            .map_err(anyhow_error("Cannot get block: {:?}"))?;

        // Convert the value to a vector of bytes.
        let bytes = Uint8Array::new(&value).to_vec();
        Ok(Cow::Owned(bytes))
    }
}

impl From<IpldCodec> for Code {
    fn from(codec: IpldCodec) -> Self {
        match codec {
            IpldCodec::DagPb => Code::DagProtobuf,
            IpldCodec::DagCbor => Code::DagCbor,
            IpldCodec::DagJson => Code::DagJson,
            IpldCodec::Raw => Code::Raw,
        }
    }
}
