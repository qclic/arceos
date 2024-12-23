use super::{property::Property, PacketError, ReadBuf, ToBytes};

pub struct Header {
    pub control_type: ControlPakcetType,
    pub remaining_len: u8,
}

impl Header {
    pub fn new() -> Self {
        Self {
            control_type: ControlPakcetType::Connect,
            remaining_len: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ControlPakcetType {
    Connect,
    ConnAck,
}

impl ControlPakcetType {
    fn to_byte(&self) -> u8 {
        let mut byte = 0;

        let mut ty = 0;
        let mut flags = 0;

        match self {
            ControlPakcetType::Connect => {
                ty = 1;
            }
            ControlPakcetType::ConnAck => {
                ty = 2;
            }
        }

        byte |= ty << 4;
        byte |= flags;
        byte
    }

    fn parse(byte: u8) -> Result<ControlPakcetType, PacketError> {
        Ok(match byte >> 4 {
            1 => ControlPakcetType::Connect,
            2 => ControlPakcetType::ConnAck,
            _ => return Err(PacketError::InvaildControlType(byte)),
        })
    }
}

impl ReadBuf for ControlPakcetType {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        Ok(())
    }
}

impl Header {
    pub fn parse(buff: &[u8]) -> Result<Header, PacketError> {
        if buff.len() < 2 {
            return Err(PacketError::BufferTooShort);
        }

        let control_type = ControlPakcetType::parse(buff[0])?;

        let remaining_len = buff[1];

        Ok(Self {
            control_type,
            remaining_len,
        })
    }
}

impl ToBytes for Header {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let mut bytes = [0; 2];

        bytes[0] = self.control_type.to_byte();
        bytes[1] = self.remaining_len;
        bytes.to_vec()
    }
}

impl ReadBuf for Header {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        let byte = buff.next().ok_or(PacketError::BufferTooShort)?;
        self.control_type = ControlPakcetType::parse(byte)?;
        self.remaining_len = buff.next().ok_or(PacketError::BufferTooShort)?;
        Ok(())
    }
}
