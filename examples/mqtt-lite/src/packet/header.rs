use alloc::vec::Vec;

use super::{data::VariableByteInt, PacketError, ReadBuf, ToBytes};

pub struct FixHeader {
    pub control_type: ControlPakcetType,
    pub remaining_len: usize,
}

impl FixHeader {
    pub fn new(ty: ControlPakcetType, remaining_len: usize) -> Self {
        Self {
            control_type: ty,
            remaining_len: remaining_len as _,
        }
    }
}

impl Default for FixHeader {
    fn default() -> Self {
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
    fn to_byte(self) -> u8 {
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

impl ToBytes for FixHeader {
    fn to_bytes(&self) -> alloc::vec::Vec<u8> {
        let t = self.control_type.to_byte();
        let mut len = VariableByteInt::from(self.remaining_len).to_bytes();
        let mut bytes = Vec::with_capacity(len.len() + 1);
        bytes.push(t);
        bytes.append(&mut len);
        bytes
    }
}

impl ReadBuf for FixHeader {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>) -> Result<(), PacketError> {
        let byte = buff.next().ok_or(PacketError::BufferTooShort)?;
        self.control_type = ControlPakcetType::parse(byte)?;

        let mut len = VariableByteInt::default();
        len.read(buff)?;

        self.remaining_len = len.to_usize();
        Ok(())
    }
}
