use alloc::vec::Vec;

use crate::{BufRead, MqttError};

use super::{data::VariableByteInt, PacketError, Reader, ToBytes};

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
    Publish { dup: bool, qos: u8, retain: bool },
}

impl ControlPakcetType {
    fn to_byte(self) -> u8 {
        let mut byte = 0;

        let ty;
        let mut flags = 0;

        match self {
            ControlPakcetType::Connect => {
                ty = 1;
            }
            ControlPakcetType::ConnAck => {
                ty = 2;
            }
            ControlPakcetType::Publish { dup, qos, retain } => {
                ty = 3;
                if dup {
                    flags |= 1 << 3;
                }
                flags |= qos << 1;
                if retain {
                    flags |= 1;
                }
            }
        }

        byte |= ty << 4;
        byte |= flags;
        byte
    }

    fn parse(byte: u8) -> Result<ControlPakcetType, MqttError> {
        Ok(match byte >> 4 {
            1 => ControlPakcetType::Connect,
            2 => ControlPakcetType::ConnAck,
            3 => {
                let dup = (byte >> 3) & 1 == 1;
                let qos = (byte >> 1) & 3;
                let retain = byte & 1 == 1;

                ControlPakcetType::Publish { dup, qos, retain }
            }
            _ => return Err(MqttError::Packet(PacketError::InvaildControlType(byte))),
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

impl Reader for FixHeader {
    fn read(&mut self, buff: &mut impl crate::BufRead) -> Result<(), crate::MqttError> {
        let byte = buff.next()?;
        self.control_type = ControlPakcetType::parse(byte)?;

        let mut len = VariableByteInt::default();
        len.read(buff)?;

        self.remaining_len = len.to_usize();
        Ok(())
    }
}
