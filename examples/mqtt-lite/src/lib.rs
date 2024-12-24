#![cfg_attr(not(test), no_std)]

extern crate alloc;

mod packet;

use core::slice::Iter;

pub use packet::{connack::ConnAck, connect::Connect, Packet};

pub use packet::{Reader, ToBytes};

#[derive(Debug)]
pub enum MqttError {
    Disconnected,
    Packet(packet::PacketError),
    EOF,
}

pub trait BufRead {
    fn read_exact(&mut self, buff: &mut [u8]) -> Result<(), MqttError>;
    fn next(&mut self) -> Result<u8, MqttError> {
        let mut buf = [0u8];
        self.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl BufRead for Iter<'_, u8> {
    fn read_exact(&mut self, buff: &mut [u8]) -> Result<(), MqttError> {
        for byte in buff.iter_mut() {
            *byte = *(Iterator::next(self).ok_or(MqttError::EOF)?);
        }
        Ok(())
    }
}
