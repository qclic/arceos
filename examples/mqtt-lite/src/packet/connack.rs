use alloc::vec::Vec;

use crate::MqttError;

use super::{data::Bits, PacketError, Reader, ToBytes};

#[derive(Default, Debug)]
pub struct ConnAck {
    pub session_present: bool,
    pub err: Option<MqttError>,
}

impl Reader for ConnAck {
    fn read(&mut self, buff: &mut impl crate::BufRead) -> Result<(), crate::MqttError> {
        let mut flags = Bits::default();
        flags.read(buff)?;

        self.session_present = flags.raw() & 0b1 != 0;

        let mut connect_reason = Bits::default();
        connect_reason.read(buff)?;
        let connect_reason_code_raw = connect_reason.raw();
        if connect_reason_code_raw != 0 {
            self.err = Some(connect_reason_code_raw.try_into()?);
        }

        Ok(())
    }
}

impl ToBytes for ConnAck {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes
    }
}

impl TryFrom<u8> for MqttError {
    type Error = MqttError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let v = match value {
            0x80 => MqttError::Unspecified,
            0x81 => MqttError::MalformedPacket,
            0x82 => MqttError::Protocol,
            0x83 => MqttError::ImplementationSpecific,
            0x84 => MqttError::UnsupportedProtocolVersion,
            0x85 => MqttError::ClientIdentifierNotValid,
            0x86 => MqttError::BadUsernameOrPassword,
            0x87 => MqttError::NotAuthorized,
            0x88 => MqttError::ServerUnavailable,
            0x89 => MqttError::ServerBusy,
            0x8a => MqttError::Banned,
            0x8c => MqttError::BadAuthenticationMethod,
            0x90 => MqttError::TopicNameInvalid,
            0x95 => MqttError::PacketTooLarge,
            0x97 => MqttError::QuotaExceeded,
            0x99 => MqttError::PayloadFormatInvalid,
            0x9a => MqttError::RetainNotSupported,
            0x9b => MqttError::QosNotSupported,
            0x9c => MqttError::UseAnotherServer,
            0x9d => MqttError::ServerMoved,
            0x9f => MqttError::ConnectionRateExceeded,
            _ => {
                return Err(MqttError::Packet(PacketError::Read(
                    "unknown connack reason code",
                )))
            }
        };

        Ok(v)
    }
}
