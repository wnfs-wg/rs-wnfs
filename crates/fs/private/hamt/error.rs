use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum HamtError {
    CursorOutOfBounds,
}

impl Error for HamtError {}

impl Display for HamtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
