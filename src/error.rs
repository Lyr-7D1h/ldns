use std::{error::Error, fmt::Display};

use bitreader::error::BitError;

#[derive(Debug)]
pub enum DnsError {
    ParseError(&'static str),
}

impl Error for DnsError {}

impl From<BitError> for DnsError {
    fn from(e: BitError) -> Self {
        match e {
            BitError::EndOfBytes => DnsError::ParseError("Unexpected end of bytes"),
            BitError::InvalidCountSize => panic!("Invalid count size"),
        }
    }
}

impl From<&'static str> for DnsError {
    fn from(e: &'static str) -> Self {
        DnsError::ParseError(e)
    }
}

impl Display for DnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DnsError::ParseError(e) => write!(f, "{e}"),
        }
    }
}
