use alloc::vec::Vec;

use super::{
    data::{FourByteInt, VariableByteInt},
    ToBytes,
};

pub enum Property {
    SessionExpiryInterval(u32),
}

impl ToBytes for Property {
    fn to_bytes(&self) -> Vec<u8> {
        let id;
        let mut value;

        match self {
            Property::SessionExpiryInterval(v) => {
                id = PropertyId::SessionExpiryInterval;
                value = FourByteInt(*v).to_bytes();
            }
        };

        let mut buf = VariableByteInt(id.into()).to_bytes();

        buf.append(&mut value);

        buf
    }
}

#[repr(u32)]
enum PropertyId {
    PayloadFormat = 0x01,
    SessionExpiryInterval = 0x11,
}

impl From<PropertyId> for u32 {
    fn from(value: PropertyId) -> Self {
        value as u32
    }
}

impl ToBytes for &[Property] {
    fn to_bytes(&self) -> Vec<u8> {
        let mut prop_bytes = Vec::new();
        for p in self.iter() {
            prop_bytes.append(&mut p.to_bytes());
        }
        let mut buff = VariableByteInt(prop_bytes.len() as _).to_bytes();

        buff.append(&mut prop_bytes);

        buff
    }
}
