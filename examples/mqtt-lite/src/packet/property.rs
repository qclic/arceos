use alloc::vec::Vec;

use super::ToBytes;

pub enum Property {}

#[repr(u8)]
enum PropertyId {
    PayloadFormat = 0x01,
    SessionExpiryInterval = 0x11,
}
