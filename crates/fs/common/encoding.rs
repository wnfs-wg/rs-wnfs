/// Helper methods for decoding and encoding values into DagCbor.
pub mod dagcbor {
    use std::io::Cursor;

    use libipld::{
        cbor::DagCborCodec,
        codec::{Decode, Encode},
        serde as ipld_serde, Ipld,
    };
    use serde::{de::DeserializeOwned, Serialize};

    /// Encodes a serializable value into DagCbor bytes.
    pub fn encode<S: Serialize>(value: &S) -> Vec<u8> {
        let ipld = ipld_serde::to_ipld(value).unwrap();
        let mut bytes = Vec::new();
        ipld.encode(DagCborCodec, &mut bytes).unwrap();
        bytes
    }

    /// Decodes recieved DagCbor bytes into a deserializable value.
    pub fn decode<D: DeserializeOwned>(bytes: &[u8]) -> D {
        let ipld = Ipld::decode(DagCborCodec, &mut Cursor::new(bytes)).unwrap();
        ipld_serde::from_ipld::<_>(ipld).unwrap()
    }
}
