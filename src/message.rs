use std::fmt::Debug;
use std::net::{Ipv4Addr, Ipv6Addr};

use bitreader::BitReader;
use rand::xorshift_u16;

use crate::error::DnsError;

mod header;
use header::Header;

mod opcode;
use opcode::*;

mod record;
use record::*;

mod question;
use question::*;

mod rcode;
use rcode::*;

fn read_name(mut bit_reader: &mut BitReader) -> Result<String, DnsError> {
    let mut name = String::new();

    // Max of 255 bytes per name: https://datatracker.ietf.org/doc/html/rfc1035#section-2.3.4
    for _ in 0..255 {
        if bit_reader.next_u8(2)? == 3 {
            let start = bit_reader.next_u8(6)? as usize;
            bit_reader.skip(2);
            let offset = bit_reader.next_u8(6)? as usize;

            let current_pointer = bit_reader.get_pointer();
            bit_reader.set_pointer(start * 8 + offset * 8);

            name.push_str(&read_name(&mut bit_reader)?);

            bit_reader.set_pointer(current_pointer);

            return Ok(name);
        }

        let char_count = bit_reader.next_u8(6)?;

        if char_count == 0 {
            break;
        }

        for _ in 0..char_count {
            let n = bit_reader.next_u8(8)?;
            name.push(n as char)
        }
        name.push('.');
    }

    Ok(name)
}

/// Read DNS Resource Records according to: https://datatracker.ietf.org/doc/html/rfc1035#section-4.1.3
fn read_record(mut bit_reader: &mut BitReader) -> Result<Record, DnsError> {
    let name = read_name(&mut bit_reader)?;

    let type_value = bit_reader.next_u16(16)?;
    let r#type = Type::from_u16(type_value).ok_or(DnsError::ParseError("Invalid type"))?;
    let class_value = bit_reader.next_u16(16)?;
    let class = Class::from_u16(class_value).expect("Invalid class");
    let ttl = bit_reader.next_u32(32)?;

    // TODO use rdlength somehow
    let _rdlength = bit_reader.next_u16(16)?;
    let rdata = match r#type {
        Type::AAAA => {
            let c = bit_reader.next_u128(128)?;
            Ipv6Addr::from(c).to_string()
        }
        Type::A => {
            let c = bit_reader.next_u32(32)?;
            Ipv4Addr::from(c).to_string()
        }
        Type::CNAME => read_name(&mut bit_reader)?,
        _ => String::new(), // TODO implement other RData formats
    };

    Ok(Record {
        name,
        r#type,
        class,
        ttl,
        rdata,
    })
}

#[derive(Debug)]
pub struct Message {
    header: Header,

    questions: Vec<Question>,
    answers: Vec<Record>,
    authority: Vec<Record>,
    additional: Vec<Record>,
}

/// DNS Message Parser following https://datatracker.ietf.org/doc/html/rfc1035
impl Message {
    pub fn query(qname: String) -> Message {
        let questions = vec![Question {
            qname: qname,
            qtype: QType::A,
            qclass: QClass::Any,
        }];

        let header = Header {
            id: xorshift_u16(),
            response: true,
            opcode: Opcode::Query,
            authenticated_data: false,
            checking_disabled: false,
            rcode: RCode::NoError,
            authoritive_answer: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            question_count: questions.len() as u16,
            answer_count: 0,
            authority_count: 0,
            additional_resource_count: 0,
        };

        Message {
            header,
            questions,
            answers: vec![],
            authority: vec![],
            additional: vec![],
        }
    }

    /// Return a byte array of the DNS Message in big-endian order
    pub fn as_bytes(&self) -> Vec<u8> {
        let bytes = self.header.as_bytes();
        return bytes;
    }

    /// Implements https://datatracker.ietf.org/doc/html/rfc1035#section-4
    pub fn from_bytes(bytes: &[u8]) -> Result<Message, DnsError> {
        let mut bit_reader = BitReader::from_bytes(bytes);
        let header = Header::from_bytes(
            &bytes[0..12]
            // bytes
            //     .take(..12)
            //     .ok_or(DnsError::ParseError("Not enough bytes for header"))?,
        )?;

        let mut questions = vec![];
        for _ in 0..header.question_count {
            questions.push(Question {
                qname: read_name(&mut bit_reader)?,
                qtype: QType::from_u16(bit_reader.next_u16(16)?)
                    .ok_or(DnsError::ParseError("Invalid qtype"))?,
                qclass: QClass::try_from(bit_reader.next_u16(16)?)?,
            });
        }

        let mut answers = vec![];
        for _ in 0..header.answer_count {
            answers.push(read_record(&mut bit_reader)?)
        }
        let mut authority = vec![];
        for _ in 0..header.authority_count {
            authority.push(read_record(&mut bit_reader)?)
        }
        let mut additional = vec![];
        for _ in 0..header.additional_resource_count {
            additional.push(read_record(&mut bit_reader)?)
        }

        Ok(Message {
            header,
            questions,
            answers,
            authority,
            additional,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn to_bytes_works() {}
}
