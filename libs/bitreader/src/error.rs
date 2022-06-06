use core::fmt;
use std::error::Error;

#[derive(Debug, Clone)]
pub enum BitError {
    EndOfBytes,
    InvalidCountSize,
}

impl Error for BitError {}

impl fmt::Display for BitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BitError::EndOfBytes => write!(f, "Reached the end of bytes"),
            BitError::InvalidCountSize => write!(f, "Bit count is too big for date type"),
        }
    }
}
