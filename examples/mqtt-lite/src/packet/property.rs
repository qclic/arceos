use alloc::{string::String, vec::Vec};

use super::{
    data::{Bits, FourByteInt, VariableByteInt},
    ToBytes,
};

pub enum Property {
    PayloadFormat(PayloadFormat),
    MessageExpiryInterval(FourByteInt),
    ContentType(String),
    ResponseTopic(String),
    CorrelationData(Vec<u8>),
    SubscriptionIdentifier(VariableByteInt),
    SessionExpiryInterval(u32),
}

pub enum PayloadFormat {
    Unspecified,
    Utf8,
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
            Property::PayloadFormat(v) => {
                id = PropertyId::PayloadFormat;

                value = Bits(match v {
                    PayloadFormat::Unspecified => 0,
                    PayloadFormat::Utf8 => 1,
                })
                .to_bytes()
            }
            Property::MessageExpiryInterval(v) => {
                id = PropertyId::MessageExpiryInterval;
                value = v.to_bytes()
            }
            Property::ContentType(v) => {
                id = PropertyId::ContentType;
                value = v.to_bytes()
            }
            Property::ResponseTopic(v) => {
                id = PropertyId::ResponseTopic;
                value = v.to_bytes()
            }
            Property::CorrelationData(vec) => todo!(),
            Property::SubscriptionIdentifier(v) => {
                id = PropertyId::SubscriptionIdentifier;
                value = v.to_bytes()
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
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0B,
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
