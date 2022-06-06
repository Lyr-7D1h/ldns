use core::fmt;
use std::fmt::Write;

use crate::util::UpperCaseFormatter;

// TODO implement https://datatracker.ietf.org/doc/html/rfc2929#section-2.2
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    Query = 0,
    // FIXME: make this outdated https://datatracker.ietf.org/doc/html/rfc3425
    IQuery = 1,
    Status = 2,
}

impl TryFrom<u8> for Opcode {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::Query),
            1 => Ok(Opcode::IQuery),
            2 => Ok(Opcode::Status),
            _ => Err("Unknown value for opcode"),
        }
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(UpperCaseFormatter(f), "{:?}", self)
    }
}
