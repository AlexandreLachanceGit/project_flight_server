#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MessageType {
    Ping,
    Data,
    Disconnect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeserializeError {
    UnsupportedMessageType,
    Format(&'static str),
}

impl TryFrom<u8> for MessageType {
    type Error = DeserializeError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(MessageType::Ping),
            1 => Ok(MessageType::Data),
            2 => Ok(MessageType::Disconnect),
            _ => Err(DeserializeError::UnsupportedMessageType),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(item: MessageType) -> u8 {
        match item {
            MessageType::Ping => 0,
            MessageType::Data => 1,
            MessageType::Disconnect => 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    header: Header,
    payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Header {
    msg_type: MessageType,
    payload_size: u16,
}

impl Header {
    const SIZE: usize = 3;
}

impl Message {
    /// Serialize a Message into a byte array
    pub fn serialize(&self) -> Vec<u8> {
        let mut res = Vec::with_capacity(Header::SIZE + self.header.payload_size as usize);

        res.push(self.header.msg_type.into());
        res.extend_from_slice(&self.header.payload_size.to_le_bytes()[0..2]);
        res.extend_from_slice(&self.payload);

        res
    }

    /// Deserialize a byte array into a Message
    pub fn deserialize(data: &[u8]) -> Result<Self, DeserializeError> {
        if data.len() < Header::SIZE {
            return Err(DeserializeError::Format("Data too short to deserialize"));
        }

        let msg_type = data[0].try_into()?;
        let payload_size = u16::from_le_bytes([data[1], data[2]]);

        if data.len() != (payload_size as usize + Header::SIZE) {
            return Err(DeserializeError::Format(
                "Data length does not match payload size",
            ));
        }
        let payload = data[Header::SIZE..].to_vec();

        Ok(Self {
            header: Header {
                msg_type,
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
        let message = Message {
            header: Header {
                msg_type: MessageType::Ping,
                payload_size: 4,
            },
            payload: vec![0xde, 0xad, 0xbe, 0xef],
        };

        let serialized = message.serialize();
        assert_eq!(serialized, vec![0, 4, 0, 0xde, 0xad, 0xbe, 0xef]);
    }

    #[test]
    /// Basic deserialization
    fn test_deserialize_valid_data() {
        let data = vec![0, 4, 0, 0xde, 0xad, 0xbe, 0xef];
        let expected = Message {
            header: Header {
                msg_type: MessageType::Ping,
                payload_size: 4,
            },
            payload: vec![0xde, 0xad, 0xbe, 0xef],
        };

        let deserialized = Message::deserialize(&data).expect("Deserialization failed");
        assert_eq!(deserialized, expected);
    }

    #[test]
    /// Message serialized and deserialized should be equal
    fn test_serialize_and_deserialize() {
        let message = Message {
            header: Header {
                msg_type: MessageType::Ping,
                payload_size: 4,
            },
            payload: vec![0xde, 0xad, 0xbe, 0xef],
        };

        let serialized = message.serialize();
        let deserialized = Message::deserialize(&serialized).expect("Deserialization failed");

        assert_eq!(deserialized, message);
    }

    #[test]
    /// Invalid data length
    fn test_deserialize_invalid_length() {
        let data = vec![0, 4];
        let result = Message::deserialize(&data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeserializeError::Format("Data too short to deserialize")
        );
    }

    #[test]
    /// Payload size says 4, but only 2 bytes provided
    fn test_deserialize_payload_size_mismatch() {
        let data = vec![0, 4, 0, 0xde, 0xad];
        let result = Message::deserialize(&data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DeserializeError::Format("Data length does not match payload size")
        );
    }
}
