use core::time::Duration;

use alloc::{string::String, vec::Vec};

use super::{
    data::{FourByteInt, TwoByteInt},
    header::{ControlPakcetType, FixHeader},
    property::{PayloadFormat, Property},
    Reader, ToBytes,
};

#[derive(Debug, Default)]
pub struct Publish {
    pub topic_name: String,
    pub packet_id: Option<u16>,
    pub payload: Payload,
    pub message_expiry_interval: Option<Duration>,
    pub response_topic: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Payload {
    Binary(Vec<u8>),
    Text(String),
}

impl Default for Payload {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl Publish {
    /// `packet_id` must be `Some` if `qos` is `1` or `2`.
    pub fn new(topic_name: impl Into<String>, payload: Payload, packet_id: Option<u16>) -> Self {
        Self {
            topic_name: topic_name.into(),
            packet_id,
            payload,
            ..Default::default()
        }
    }
}

impl ToBytes for Publish {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.topic_name.to_bytes());

        if let Some(packet_id) = self.packet_id {
            bytes.extend(TwoByteInt(packet_id).to_bytes());
        }

        let payload_buff;

        let fmt = match &self.payload {
            Payload::Binary(vec) => {
                payload_buff = vec.to_bytes();
                PayloadFormat::Unspecified
            }
            Payload::Text(t) => {
                payload_buff = t.to_bytes();
                PayloadFormat::Utf8
            }
        };

        let mut properties = alloc::vec![Property::PayloadFormat(fmt),];

        if let Some(interval) = self.message_expiry_interval {
            properties.push(Property::MessageExpiryInterval(FourByteInt(
                interval.as_secs() as u32,
            )));
        }

        if let Some(topic) = self.response_topic.as_ref() {
            properties.push(Property::ResponseTopic(topic.clone()));
        }

        bytes.extend((&properties[..]).to_bytes());

        bytes.extend(payload_buff);

        bytes
    }
}

impl Reader for Publish {
    fn read(&mut self, buff: &mut impl crate::BufRead) -> Result<(), crate::MqttError> {
        todo!()
    }
}
