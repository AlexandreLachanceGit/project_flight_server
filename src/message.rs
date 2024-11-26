#[derive(Debug)]
enum MessageType {
    Ping,
    Data,
    Disconnect,
    Unsupported,
}

impl Into<MessageType> for u8 {
    fn into(self) -> MessageType {
        match self {
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

#[derive(Debug)]
struct Message {
    msg_type: MessageType,
    payload: Vec<u8>,
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        todo!()
    }

    pub fn deserialize(buffer: &[u8]) -> Self {
        todo!()
    }
}
