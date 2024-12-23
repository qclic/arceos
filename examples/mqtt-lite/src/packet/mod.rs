pub mod connect;
pub mod header;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MsgTypes {
    Connect = 1,
    Connack,
    Publish,
    Puback,
    Pubrec,
    Pubrel,
    Pubcomp,
    Subscribe,
    Suback,
    Unsubscribe,
    Unsuback,
    Pingreq,
    Pingresp,
    Disconnect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketError {
    BufferTooShort,
    Read,
    InvaildControlType(u8),
}

#[repr(C)]
pub struct MQTTLenString {
    pub len: usize,
    pub data: *const char,
}

#[repr(C)]
pub struct MQTTString {
    pub cstring: *const char,
    pub lenstring: MQTTLenString,
}

impl MQTTString {
    pub fn new() -> Self {
        MQTTString {
            cstring: core::ptr::null(),
            lenstring: MQTTLenString {
                len: 0,
                data: core::ptr::null(),
            },
        }
    }

    pub fn len(&self) -> usize {
        self.lenstring.len
    }
}

impl Default for MQTTString {
    fn default() -> Self {
        Self::new()
    }
}
