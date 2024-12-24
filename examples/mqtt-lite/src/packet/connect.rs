use core::time::Duration;

use alloc::{string::String, vec::Vec};

use super::{
    header::{ControlPakcetType, FixHeader}, property::Property, Reader, ToBytes
};

pub struct Connect {
    pub protocol_level: u8,
    pub client_id: String,
    pub clean_start: bool,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub keep_alive: Duration,
    pub session_expiry_interval_sec: u32,
}
impl Default for Connect {
    fn default() -> Self {
        Self {
            protocol_level: 5,
            clean_start: Default::default(),
            user_name: Default::default(),
            password: Default::default(),
            keep_alive: Default::default(),
            session_expiry_interval_sec: Default::default(),
            client_id: Default::default(),
        }
    }
}

impl Connect {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            client_id: id.into(),
            protocol_level: 5,
            ..Default::default()
        }
    }
}

impl ToBytes for Connect {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend(ProtocolName::to_bytes());
        buf.push(self.protocol_level);

        let mut flag = ConnectFlags::empty();

        if self.clean_start {
            flag |= ConnectFlags::CLEAN_START;
        }
        if self.user_name.is_some() {
            flag |= ConnectFlags::USER_NAME;
        }
        if self.password.is_some() {
            flag |= ConnectFlags::PASSWORD;
        }

        buf.push(flag.bits());

        let keep_alive = self.keep_alive.as_secs() as u16;
        buf.extend(keep_alive.to_be_bytes());

        let properties = alloc::vec![Property::SessionExpiryInterval(
            self.session_expiry_interval_sec
        ),];

        buf.extend((&properties[..]).to_bytes());

        // Payload

        let client_id = self.client_id.to_bytes();
        buf.extend(client_id);

        //TODO: will properties

        //TODO: will topic

        //TODO: will payload

        if let Some(user_name) = self.user_name.as_ref() {
            buf.extend(user_name.to_bytes());
        }

        if let Some(password) = self.password.as_ref() {
            buf.extend(password.to_bytes());
        }

        buf
    }
}


impl Reader for Connect {
    fn read(&mut self, buff: &mut impl crate::BufRead) -> Result<(), crate::MqttError> {
        todo!()
    }
}

struct ProtocolName {}

impl ProtocolName {
    fn to_bytes() -> [u8; 6] {
        [0, 4, b'M', b'Q', b'T', b'T']
    }
}

bitflags::bitflags! {
    pub struct ConnectFlags: u8 {
        const USER_NAME = 1 << 7;
        const PASSWORD = 1 << 6;
        const WILL_RETAIN = 1 << 5;
        const WILL_QOS = 1 << 3;
        const WILL_FLAG = 1 << 2;
        const CLEAN_START = 1 << 1;
        const RESERVED = 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connect() {
        let mut connect = Connect::default();
        connect.clean_start = true;
        connect.keep_alive = Duration::from_secs(10);
        connect.session_expiry_interval_sec = 10;
        let bytes = connect.to_bytes();
        println!("{:?}", bytes);
    }
}
