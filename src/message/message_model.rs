use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub enum EventType {
    Login,
    Msg,
    Heart,
    Logout,
    Ack,
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Text,
    RichText,
    Image,
}

#[derive(Serialize, Deserialize)]
pub struct MsgBody {
    pub kind: MessageType,
    pub uid: u64,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct MsgEvent {
    pub event: EventType,
    pub body: MsgBody,
}

