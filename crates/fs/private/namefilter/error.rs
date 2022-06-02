use std::{error::Error, fmt::Display};

/// Namefilter errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NameFilterError {
    MaxItemCountReached,
}

impl Error for NameFilterError {}

impl Display for NameFilterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
