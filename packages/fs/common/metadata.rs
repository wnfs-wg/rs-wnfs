use semver::Version;

pub enum UnixNodeKind {
    // See https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
    Raw,
    File,
    Dir,
    Metadata,
    SymLink,
    HAMTShard,
}

pub struct UnixMetadata {
    // See https://docs.ipfs.io/concepts/file-systems/#unix-file-system-unixfs
    mtime: u64,
    ctime: u64,
    mode: u32,
    kind: UnixNodeKind,
}

pub struct Metadata {
    unix_metadata: UnixMetadata,
    is_file: bool, // TODO: Already in UnixMetadata?
    version: Version,
}
