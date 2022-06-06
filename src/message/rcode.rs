use core::fmt;
use std::fmt::Write;

use crate::util::UpperCaseFormatter;

// TODO implement https://datatracker.ietf.org/doc/html/rfc2929#section-2.3
#[derive(Debug, Clone, Copy)]
pub enum RCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}
impl RCode {
    pub fn from_u8(value: u8) -> Option<RCode> {
        match value {
            0 => Some(RCode::NoError),
            1 => Some(RCode::FormatError),
            2 => Some(RCode::ServerFailure),
            3 => Some(RCode::NameError),
            4 => Some(RCode::NotImplemented),
            5 => Some(RCode::Refused),
            _ => None,
        }
    }
}
impl fmt::Display for RCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(UpperCaseFormatter(f), "{:?}", self)
    }
}
