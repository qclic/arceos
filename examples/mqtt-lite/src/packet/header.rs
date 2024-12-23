use super::PacketError;

pub struct Header {
    pub control_type: ControlPakcetType,
    pub remaining_len: u8,
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

impl Header {
    pub fn to_bytes(&self) -> [u8; 2] {
        let mut bytes = [0; 2];

        bytes[0] = self.control_type.to_byte();
        bytes[1] = self.remaining_len;
        bytes
    }

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
