use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum Message {
    Ping,
    Data,
    Disconnect,
    Chat,
}

#[derive(Error, Debug)]
pub enum DeserializeError {
    #[error("message type not supported")]
    UnsupportedMessageType,
    #[error("message too short for headers")]
    MessageTooShort,
    #[error("data length doesn't match payload size")]
    InvalidPayloadLength,
    #[error("failed to deserialize payload")]
    DeserializePayload(#[from] bincode::Error),
}

impl TryFrom<u8> for Message {
    type Error = DeserializeError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Message::Ping),
            1 => Ok(Message::Data),
            2 => Ok(Message::Disconnect),
            3 => Ok(Message::Chat),
            _ => Err(DeserializeError::UnsupportedMessageType),
        }
    }
}

impl From<Message> for u8 {
    fn from(item: Message) -> u8 {
        match item {
            Message::Ping => 0,
            Message::Data => 1,
            Message::Disconnect => 2,
            Message::Chat => 3,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MessagePacket {
    header: Header,
    payload: Message,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Header {
    timestamp: u32,
    version: u16,
    payload_size: u16,
}

impl Header {
    const SIZE: usize = 8;
    const CURRENT_VERSION: u16 = 0;
}

impl MessagePacket {
    pub fn new(message: Message) -> MessagePacket {
        MessagePacket {
            header: Header {
                timestamp: crate::time::get_time(),
                version: 0,
                payload_size: 0, // overwritten at serialization
            },
            payload: message,
        }
    }

    /// Serialize a Message into a byte array
    pub fn serialize(&self) -> Vec<u8> {
        let payload = &bincode::serialize(&self.payload).unwrap();

        let mut res = Vec::with_capacity(Header::SIZE + payload.len());

        res.extend_from_slice(&self.header.timestamp.to_le_bytes());
        res.extend_from_slice(&self.header.version.to_le_bytes());
        res.extend_from_slice(&(payload.len() as u16).to_le_bytes());
        res.extend_from_slice(payload);

        res
    }

    /// Deserialize a byte array into a Message
    pub fn deserialize(data: &[u8]) -> Result<Self, DeserializeError> {
        if data.len() < Header::SIZE {
            return Err(DeserializeError::MessageTooShort);
        }

        let timestamp = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
        let version = u16::from_le_bytes([data[4], data[5]]);
        let payload_size = u16::from_le_bytes([data[6], data[7]]);

        if data.len() != (payload_size as usize + Header::SIZE) {
            return Err(DeserializeError::InvalidPayloadLength);
        }
        let payload_bytes = &data[Header::SIZE..];
        let payload = bincode::deserialize(payload_bytes)?;

        Ok(Self {
            header: Header {
                timestamp,
                version,
                payload_size,
            },
            payload,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Basic serialization
    fn test_serialize() {
        let message = Message::Ping;

        let mut serialized_payload = bincode::serialize(&message).unwrap();
        let message = MessagePacket {
            header: Header {
                timestamp: 12345678,
                version: 1,
                payload_size: serialized_payload.len() as u16,
            },
            payload: message,
        };

        #[rustfmt::skip]
        let mut correct_serialized: Vec<u8> = vec![
                78, 97, 188, 0, // timestamp (12345678 in little-endian)
                1, 0, // version (1 in little-endian)
                serialized_payload.len() as u8, 0, // payload size 
            ];
        correct_serialized.append(&mut serialized_payload); // payload

        // The expected serialized format includes the header and the payload.
        assert_eq!(message.serialize(), correct_serialized);
    }

    #[test]
    /// Message serialized and deserialized should be equal
    fn test_serialize_and_deserialize() {
        let message = MessagePacket {
            header: Header {
                timestamp: 12345678,
                version: 1,
                payload_size: 4,
            },
            payload: Message::Ping,
        };

        let serialized = message.serialize();
        let deserialized = MessagePacket::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(deserialized, message);
    }

    #[test]
    /// Invalid data length
    fn test_deserialize_invalid_length() {
        let data = vec![
            78, 97, 188, 0, // timestamp (12345678 in little-endian)
            1, 0, // version (1 in little-endian)
            4, 0, // payload size (4 in little-endian)
        ];
        let result = MessagePacket::deserialize(&data);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DeserializeError::InvalidPayloadLength
        ));
    }

    #[test]
    /// Payload size says 4, but only 2 bytes provided
    fn test_deserialize_payload_size_mismatch() {
        let data = vec![
            78, 97, 188, 0, // timestamp (12345678 in little-endian)
            1, 0, // version (1 in little-endian)
            4, 0, // payload size (4 in little-endian)
            0, 1, // Incomplete payload (only 2 bytes instead of 4)
        ];
        let result = MessagePacket::deserialize(&data);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            DeserializeError::InvalidPayloadLength
        ));
    }
}
