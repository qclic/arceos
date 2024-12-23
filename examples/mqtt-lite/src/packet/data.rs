use alloc::{boxed::Box, string::String, vec::Vec};

use super::{PacketError, ReadBuf, ToBytes};

#[derive(Default)]
#[repr(transparent)]
pub struct Bits(u8);

impl Bits {
    pub fn raw(&self) -> u8 {
        self.0
    }
}

impl ToBytes for Bits {
    fn to_bytes(&self) -> Vec<u8> {
        alloc::vec![self.0]
    }
}

impl ReadBuf for Bits {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        self.0 = buff.next().ok_or(PacketError::BufferTooShort)?;
        Ok(())
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct TwoByteInt(u16);

impl TwoByteInt {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
    pub fn raw(&self) -> u16 {
        self.0
    }
}

impl ToBytes for TwoByteInt {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

impl ReadBuf for TwoByteInt {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        self.0 = u16::from_be_bytes([
            buff.next().ok_or(PacketError::BufferTooShort)?,
            buff.next().ok_or(PacketError::BufferTooShort)?,
        ]);

        Ok(())
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct FourByteInt(u32);

impl FourByteInt {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
    pub fn raw(&self) -> u32 {
        self.0
    }
}

impl ReadBuf for FourByteInt {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        self.0 = u32::from_be_bytes([
            buff.next().ok_or(PacketError::BufferTooShort)?,
            buff.next().ok_or(PacketError::BufferTooShort)?,
            buff.next().ok_or(PacketError::BufferTooShort)?,
            buff.next().ok_or(PacketError::BufferTooShort)?,
        ]);

        Ok(())
    }
}

impl ToBytes for FourByteInt {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }
}

#[derive(Default)]
#[repr(transparent)]
pub struct VariableByteInt(pub u32);

impl VariableByteInt {
    pub fn to_usize(&self) -> usize {
        self.0 as usize
    }
}

impl From<usize> for VariableByteInt {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl ReadBuf for VariableByteInt {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        let mut value = 0;
        let mut multiplier = 1;

        loop {
            let encoded_byte = buff.next().ok_or(PacketError::BufferTooShort)?;

            value += (encoded_byte & 127) as u32 * multiplier;

            if multiplier > 128 * 128 * 128 {
                return Err(PacketError::MalformedVariableByteInteger);
            }

            multiplier *= 128;

            if (encoded_byte & 128) == 0 {
                break;
            }
        }

        self.0 = value;
        Ok(())
    }
}

impl ToBytes for VariableByteInt {
    fn to_bytes(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(4);
        let mut x = self.0;

        loop {
            let mut encoded_byte = (x % 128) as u8;
            x /= 128;

            if x > 0 {
                encoded_byte |= 128;
            }

            output.push(encoded_byte);

            if x == 0 {
                break;
            }
        }

        output
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        let data = self.as_bytes();
        let len = data.len() as u16;

        let mut bytes = Vec::with_capacity(data.len() + 2);
        bytes.extend(len.to_be_bytes());
        bytes.extend(data);
        bytes
    }
}

impl ReadBuf for String {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        let mut len = TwoByteInt::default();
        len.read(buff)?;
        let len = len.to_usize();

        let mut data = alloc::vec![0; len];

        for i in data.iter_mut() {
            *i = buff.next().ok_or(PacketError::BufferTooShort)?;
        }

        *self = String::from_utf8(data).map_err(|_| PacketError::InvalidUtf8)?;

        Ok(())
    }
}

pub struct StringPair {
    pub key: String,
    pub value: String,
}

impl ToBytes for StringPair {
    fn to_bytes(&self) -> Vec<u8> {
        let k = self.key.to_bytes();
        let v = self.value.to_bytes();

        let mut bytes = Vec::with_capacity(k.len() + v.len());
        bytes.extend(k);
        bytes.extend(v);
        bytes
    }
}

impl ReadBuf for StringPair {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        self.key.read(buff)?;
        self.value.read(buff)?;

        Ok(())
    }
}

pub struct Praser {
    pub struct_list: Vec<Box<dyn ToBytes>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_variable_byte_int() {
        let v = VariableByteInt::from(268_435_455);

        let bytes = v.to_bytes();

        let mut v2 = VariableByteInt::default();

        let mut buff = bytes.into_iter();

        v2.read(&mut buff).unwrap();

        assert_eq!(v.0, v2.0);
    }
    #[test]
    fn test_variable_byte_int1() {
        let v = VariableByteInt::from(127);

        let bytes = v.to_bytes();

        let want = [0x7F];

        assert_eq!(&bytes, &want);
    }
    #[test]
    fn test_variable_byte_int2() {
        let v = VariableByteInt::from(16_383);

        let bytes = v.to_bytes();

        let want = [0xFF, 0x7F];

        assert_eq!(&bytes, &want);
    }
}
