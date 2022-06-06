use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, TcpStream},
};

use crate::message::Message;

pub struct Resolver;

// https://datatracker.ietf.org/doc/html/rfc1035#section-7
// https://datatracker.ietf.org/doc/html/rfc1034
impl Resolver {
    pub fn lookup(name: &str) -> IpAddr {
        let mut stream = TcpStream::connect("1.1.1.1:53").unwrap();

        let bytes = Message::query(String::from(name)).as_bytes();

        stream.write(&bytes).unwrap();

        IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1))
    }
}
