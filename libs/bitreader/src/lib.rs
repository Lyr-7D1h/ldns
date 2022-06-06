pub mod error;

use error::BitError;

pub struct BitReader<'n> {
    pointer: usize,
    bytes: &'n [u8],
}

/// An abstraction over a vector of bytes to easily navigate over the bits within the vector of bytes.
impl<'n> BitReader<'n> {
    pub fn from_bytes(bytes: &[u8]) -> BitReader {
        BitReader { bytes, pointer: 0 }
    }

    pub fn skip(&mut self, count: usize) {
        self.pointer += count;
    }

    pub fn set_pointer(&mut self, offset: usize) {
        self.pointer = offset;
    }

    pub fn get_pointer(&self) -> usize {
        self.pointer
    }

    pub fn next_bit(&mut self) -> Result<bool, BitError> {
        // Fetch current byte
        let byte = match self.bytes.get(self.pointer / 8) {
            Some(x) => x,
            None => return Err(BitError::EndOfBytes),
        };

        // Fetch first bit by ANDing a bitmask with the position of the pointer
        let bit = (byte & 1 << 7 - ((self.pointer) % 8)) > 0;
        self.pointer += 1;

        return Ok(bit);
    }

    pub fn next_u8(&mut self, bit_count: usize) -> Result<u8, BitError> {
        if bit_count > 16 {
            return Err(BitError::InvalidCountSize);
        }

        let mut value = 0;
        for i in self.pointer..self.pointer + bit_count {
            let byte = match self.bytes.get(self.pointer / 8) {
                Some(x) => x,
                None => return Err(BitError::EndOfBytes),
            };
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u8 & 1;
            value = (value << 1) | bit;
        }

        self.pointer += bit_count;

        Ok(value)
    }

    pub fn next_u16(&mut self, bit_count: usize) -> Result<u16, BitError> {
        if bit_count > 16 {
            return Err(BitError::InvalidCountSize);
        }

        let mut value = 0;
        for i in self.pointer..self.pointer + bit_count {
            let byte = match self.bytes.get(self.pointer / 8) {
                Some(x) => x,
                None => return Err(BitError::EndOfBytes),
            };
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u16 & 1;
            value = (value << 1) | bit;
        }

        self.pointer += bit_count;

        Ok(value)
    }

    pub fn next_u32(&mut self, bit_count: usize) -> Result<u32, BitError> {
        if bit_count > 16 {
            return Err(BitError::InvalidCountSize);
        }

        let mut value = 0;
        for i in self.pointer..self.pointer + bit_count {
            let byte = match self.bytes.get(self.pointer / 8) {
                Some(x) => x,
                None => return Err(BitError::EndOfBytes),
            };
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u32 & 1;
            value = (value << 1) | bit;
        }

        self.pointer += bit_count;

        Ok(value)
    }

    // Not used but implented anyways
    pub fn _next_u64(&mut self, bit_count: usize) -> Result<u64, BitError> {
        if bit_count > 16 {
            return Err(BitError::InvalidCountSize);
        }

        let mut value = 0;
        for i in self.pointer..self.pointer + bit_count {
            let byte = match self.bytes.get(self.pointer / 8) {
                Some(x) => x,
                None => return Err(BitError::EndOfBytes),
            };
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u64 & 1;
            value = (value << 1) | bit;
        }

        self.pointer += bit_count;

        Ok(value)
    }

    pub fn next_u128(&mut self, bit_count: usize) -> Result<u128, BitError> {
        if bit_count > 16 {
            return Err(BitError::InvalidCountSize);
        }

        let mut value = 0;
        for i in self.pointer..self.pointer + bit_count {
            let byte = match self.bytes.get(self.pointer / 8) {
                Some(x) => x,
                None => return Err(BitError::EndOfBytes),
            };
            let shift = 7 - (i % 8);
            let bit = (byte >> shift) as u128 & 1;
            value = (value << 1) | bit;
        }

        self.pointer += bit_count;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::BitReader;

    #[test]
    fn next_bit_works() {
        let data = vec![160, 29];
        let mut r = BitReader::from_bytes(&data);

        let mut values = vec![];
        for _ in 0..16 {
            values.push(r.next_bit().unwrap());
        }

        assert_eq!(
            values,
            vec![
                true, false, true, false, false, false, false, false, false, false, false, true,
                true, true, false, true
            ]
        );
    }

    #[test]
    fn next_u8_works() {
        let data = vec![20, 255];
        let mut r = BitReader::from_bytes(&data);
        assert_eq!(r.next_u8(8).unwrap(), 20)
    }
}
