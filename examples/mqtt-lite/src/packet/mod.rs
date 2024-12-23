use alloc::vec::Vec;

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
}

pub enum Packet {
    Connect(connect::Property),
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait ReadBuf {
    fn read(&mut self, buff: &mut impl Iterator<Item = u8>)->Result<(), PacketError>;
}

