use bitreader::BitReader;

use crate::error::DnsError;

use super::{opcode::Opcode, rcode::RCode};

/// A DNS Message Header
//  0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                      ID                       |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |QR|   Opcode  |AA|TC|RD|RA| Z|AD|CD|   RCODE   |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    QDCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    ANCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    NSCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
// |                    ARCOUNT                    |
// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
#[derive(Debug)]
pub struct Header {
    pub id: u16,
    /// If false it is a query otherwise a response
    pub response: bool,
    pub opcode: Opcode,
    pub authoritive_answer: bool,
    pub truncated: bool,
    pub recursion_desired: bool,
    pub recursion_available: bool,

    // https://www.rfc-editor.org/rfc/rfc2535#section-6.1
    pub authenticated_data: bool,
    pub checking_disabled: bool,

    pub rcode: RCode,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_resource_count: u16,
}

impl Header {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.id.to_be_bytes());

        let mut byte: u8 = 0;

        if self.response {
            byte = 1;
        }
        byte <<= 1;

        byte += self.opcode as u8;
        byte <<= 4;

        if self.authoritive_answer {
            byte += 1;
        }
        byte <<= 1;

        if self.truncated {
            byte += 1;
        }
        byte <<= 1;

        if self.recursion_desired {
            byte += 1;
        }

        bytes.push(byte);
        byte = 0;

        if self.recursion_available {
            byte = 1;
        }
        byte <<= 2; // Recursion Available + Reserved space

        if self.authenticated_data {
            byte += 1;
        }
        byte <<= 1;

        if self.checking_disabled {
            byte += 1;
        }
        byte <<= 1;

        byte += self.rcode as u8;
        bytes.push(byte);

        bytes.extend(self.question_count.to_be_bytes());
        bytes.extend(self.answer_count.to_be_bytes());
        bytes.extend(self.authority_count.to_be_bytes());
        bytes.extend(self.additional_resource_count.to_be_bytes());

        return bytes;
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Header, DnsError> {
        let mut bit_reader = BitReader::from_bytes(bytes);

        let id = bit_reader.next_u16(16)?;
        let response = bit_reader.next_bit()?;

        let opcode = Opcode::try_from(bit_reader.next_u8(4)?)?;

        let authoritive_answer = bit_reader.next_bit()?;
        let truncated = bit_reader.next_bit()?;
        let recursion_desired = bit_reader.next_bit()?;
        let recursion_available = bit_reader.next_bit()?;

        bit_reader.skip(1);

        let authenticated_data = bit_reader.next_bit()?;
        let checking_disabled = bit_reader.next_bit()?;

        let rcode =
            RCode::from_u8(bit_reader.next_u8(4)?).ok_or(DnsError::ParseError("Invalid rcode"))?;

        let question_count = bit_reader.next_u16(16)?;
        let resource_count = bit_reader.next_u16(16)?;
        let authority_count = bit_reader.next_u16(16)?;
        let additional_resource_count = bit_reader.next_u16(16)?;

        return Ok(Header {
            id,
            response,
            opcode,
            authoritive_answer,
            truncated,
            recursion_desired,
            recursion_available,
            authenticated_data,
            checking_disabled,
            rcode,
            question_count,
            answer_count: resource_count,
            authority_count,
            additional_resource_count,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_bytes_works() {
        let header = Header {
            id: 0,
            response: true,
            opcode: Opcode::Query,
            authenticated_data: false,
            checking_disabled: false,
            rcode: RCode::NoError,
            authoritive_answer: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            question_count: 1,
            answer_count: 0,
            authority_count: 0,
            additional_resource_count: 0,
        };

        let bytes = header.as_bytes();

        for byte in bytes.iter() {
            println!("{byte:#b}");
        }

        assert!(bytes, vec![0b0, 0b0, 0b100000000])
    }
}
