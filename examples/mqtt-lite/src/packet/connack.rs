use alloc::vec::Vec;

use super::{PacketError, Reader, ToBytes};

#[derive(Default, Debug)]
pub struct ConnAck {
    pub protocol_level: u8,
}

impl Reader for ConnAck {
    fn read(&mut self, buff: &mut impl crate::BufRead) -> Result<(), crate::MqttError> {
        Ok(())
    }
}

impl ToBytes for ConnAck {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes
    }
}
