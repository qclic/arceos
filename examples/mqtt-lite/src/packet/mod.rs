use alloc::vec::Vec;

use header::{ControlPakcetType, FixHeader};

use crate::{BufRead, MqttError};

pub mod connack;
pub mod connect;
mod data;
pub mod header;
pub mod property;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    BufferTooShort,
    Read,
    InvaildControlType(u8),
    InvalidUtf8,
    MalformedVariableByteInteger,
}

pub enum Packet {
    Connect(connect::Connect),
    ConnAck(connack::ConnAck),
}

impl ToBytes for Packet {
    fn to_bytes(&self) -> Vec<u8> {
        let ty;
        let mut buf;
        match self {
            Packet::Connect(connect) => {
                buf = connect.to_bytes();
                ty = ControlPakcetType::Connect;
            }
            Packet::ConnAck(conn_ack) => {
                buf = conn_ack.to_bytes();
                ty = ControlPakcetType::ConnAck;
            }
        };

        let len = buf.len();
        let header = FixHeader::new(ty, len);

        let mut all = header.to_bytes();
        all.append(&mut buf);
        all
    }
}

impl Packet {
    pub fn read_from<R: BufRead>(buff: &mut R) -> Result<Self, MqttError> {
        let mut fix_header = FixHeader::default();
        fix_header.read(buff)?;

        let mut body = alloc::vec![0; fix_header.remaining_len];

        buff.read_exact(&mut body)?;

        let mut buff = body.iter();

        match fix_header.control_type {
            ControlPakcetType::Connect => {
                let mut connect = connect::Connect::default();
                connect.read(&mut buff)?;
                Ok(Packet::Connect(connect))
            }
            ControlPakcetType::ConnAck => {
                let mut conn_ack = connack::ConnAck::default();
                conn_ack.read(&mut buff)?;
                Ok(Packet::ConnAck(conn_ack))
            }
        }
    }
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Reader {
    fn read(&mut self, buff: &mut impl BufRead) -> Result<(), MqttError>;
}
