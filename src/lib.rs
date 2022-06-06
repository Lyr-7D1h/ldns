pub mod message;

pub mod util;

pub mod error;

pub mod resolver;

pub type Result<T> = core::result::Result<T, error::DnsError>;
