//! Private node system in-memory representation.

use std::rc::Rc;

use crate::FsError;

use super::{PrivateDirectory, PrivateFile};

use anyhow::{bail, Result};

/// A node in a WNFS private file system. This can either be a private file or a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrivateNode {
    File(Rc<PrivateFile>),
    Dir(Rc<PrivateDirectory>),
}

impl PrivateNode {
    pub fn as_dir(&self) -> Result<Rc<PrivateDirectory>> {
        Ok(match self {
            Self::Dir(dir) => Rc::clone(dir),
            _ => bail!(FsError::NotADirectory),
        })
    }

    pub fn as_file(&self) -> Result<Rc<PrivateFile>> {
        Ok(match self {
            Self::File(file) => Rc::clone(file),
            _ => bail!(FsError::NotAFile),
        })
    }
}
