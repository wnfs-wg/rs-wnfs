use bytes::Bytes;
use prost::{Enumeration, Message};

/// An IPFS MerkleDAG Link
#[derive(Clone, PartialEq, Eq, Message)]
pub struct PbLink {
    /// multihash of the target object
    #[prost(bytes = "vec", optional, tag = "1")]
    pub hash: Option<Vec<u8>>,
    /// utf string name. should be unique per object
    #[prost(string, optional, tag = "2")]
    pub name: Option<String>,
    /// cumulative size of target object
    #[prost(uint64, optional, tag = "3")]
    pub tsize: Option<u64>,
}

/// An IPFS MerkleDAG Node
#[derive(Clone, PartialEq, Eq, Message)]
pub struct PbNode {
    /// refs to other objects
    #[prost(message, repeated, tag = "2")]
    pub links: Vec<PbLink>,
    /// opaque user data
    #[prost(bytes = "bytes", optional, tag = "1")]
    pub data: Option<Bytes>,
}

#[derive(Clone, PartialEq, Eq, Message)]
pub struct Data {
    #[prost(enumeration = "DataType", tag = "1")]
    pub r#type: i32,
    #[prost(bytes = "bytes", optional, tag = "2")]
    pub data: Option<Bytes>,
    #[prost(uint64, optional, tag = "3")]
    pub filesize: Option<u64>,
    #[prost(uint64, repeated, tag = "4")]
    pub blocksizes: Vec<u64>,
    #[prost(uint64, optional, tag = "5")]
    pub hash_type: Option<u64>,
    #[prost(uint64, optional, tag = "6")]
    pub fanout: Option<u64>,
}
/// Nested message and enum types in `Data`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
#[repr(i32)]
pub enum DataType {
    Raw = 0,
    Directory = 1,
    File = 2,
    Metadata = 3,
    Symlink = 4,
    HamtShard = 5,
}

impl DataType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DataType::Raw => "Raw",
            DataType::Directory => "Directory",
            DataType::File => "File",
            DataType::Metadata => "Metadata",
            DataType::Symlink => "Symlink",
            DataType::HamtShard => "HAMTShard",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> Option<Self> {
        match value {
            "Raw" => Some(Self::Raw),
            "Directory" => Some(Self::Directory),
            "File" => Some(Self::File),
            "Metadata" => Some(Self::Metadata),
            "Symlink" => Some(Self::Symlink),
            "HAMTShard" => Some(Self::HamtShard),
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Message)]
pub struct Metadata {
    #[prost(string, optional, tag = "1")]
    pub mime_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use prost::Message;
    use testresult::TestResult;

    #[test]
    fn test_parse_data_example() -> TestResult {
        let examples = [
            "CAIYy4USIICAECDLhQI",
            "CAIYwtqZKSCAgOAVIMLauRM",
            "CAIYgIDgFSCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAECCAgBAggIAQIICAEA",
        ];
        for example in examples {
            let data = data_encoding::BASE64URL_NOPAD.decode(example.as_ref())?;
            let d = Data::decode(data.as_ref())?;

            assert_eq!(d.blocksizes.iter().sum::<u64>(), d.filesize.unwrap());

            println!("{d:#?}");
        }

        Ok(())
    }
}
