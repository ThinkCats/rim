use serde::{Deserialize, Serialize};

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
    pub gid: Option<u64>,
    pub content: String,
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct MsgEvent {
    pub event: EventType,
    pub body: MsgBody,
}

#[derive(Serialize, Deserialize)]
pub struct MsgAck {
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
    pub kind: EventType,
    pub content: String,
}

impl MsgAck {
    pub fn ack(client_msg_id: String) -> MsgAck {
        MsgAck {
            client_msg_id,
            kind: EventType::Ack,
            content: "Ok".into(),
        }
    }
}
