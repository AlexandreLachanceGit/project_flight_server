#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MessageType {
    Ping,
    Data,
    Disconnect,
    Unsupported,
}

impl From<u8> for MessageType {
    fn from(val: u8) -> Self {
        match val {
            0 => MessageType::Ping,
            1 => MessageType::Data,
            2 => MessageType::Disconnect,
            _ => MessageType::Unsupported,
        }
    }
}

impl From<MessageType> for u8 {
    fn from(item: MessageType) -> u8 {
        match item {
            MessageType::Ping => 0,
            MessageType::Data => 1,
            MessageType::Disconnect => 2,
            MessageType::Unsupported => u8::MAX,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Message {
    msg_type: MessageType,
    payload_size: u16,
    payload: Vec<u8>,
}

impl Message {
    /// Serialize a Message into a byte array
    pub fn serialize(&self) -> Vec<u8> {
        // Initialize vector with header size + payload size
        let mut res = Vec::with_capacity(3 + self.payload_size as usize);

        res.push(self.msg_type.into());
        res.extend_from_slice(&self.payload_size.to_le_bytes()[0..2]);
        res.extend_from_slice(&self.payload);

        res
    }

    /// Deserialize a byte array into a Message
    pub fn deserialize(data: &[u8]) -> Result<Self, String> {
        // Check if the input data is at least 3 bytes (header size)
        if data.len() < 3 {
            return Err("Data too short to deserialize".into());
        }

        let msg_type = data[0].into();
        let payload_size = u16::from_le_bytes([data[1], data[2]]);

        if data.len() != (payload_size as usize + 3) {
            return Err("Data length does not match payload size".into());
        }
        let payload = data[3..].to_vec();

        Ok(Self {
            msg_type,
            payload_size,
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
            msg_type: MessageType::Ping,
            payload_size: 4,
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
            msg_type: MessageType::Ping,
            payload_size: 4,
            payload: vec![0xde, 0xad, 0xbe, 0xef],
        };

        let deserialized = Message::deserialize(&data).expect("Deserialization failed");
        assert_eq!(deserialized, expected);
    }

    #[test]
    /// Message serialized and deserialized should be equal
    fn test_serialize_and_deserialize() {
        let message = Message {
            msg_type: MessageType::Ping,
            payload_size: 4,
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
        assert_eq!(result.unwrap_err(), "Data too short to deserialize");
    }

    #[test]
    /// Payload size says 4, but only 2 bytes provided
    fn test_deserialize_payload_size_mismatch() {
        let data = vec![0, 4, 0, 0xde, 0xad];
        let result = Message::deserialize(&data);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Data length does not match payload size"
        );
    }
}
