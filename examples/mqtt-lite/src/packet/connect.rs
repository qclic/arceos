use core::time::Duration;

use alloc::{string::String, vec::Vec};

pub struct Property {
    struct_id: [u8; 4],
    struct_version: isize,
    mqtt_version: u8,
}

pub struct Connect {
    pub protocol_level: u8,
    pub clean_start: bool,
    pub user_name: Option<String>,
    pub password: Option<String>,
    pub keep_alive: Duration,
}

impl Connect {
    pub fn to_bytes(&self) -> Vec<u8> {
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

        


        buf
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
