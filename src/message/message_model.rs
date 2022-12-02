use std::fmt::Display;

use chrono::Local;
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

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Text => write!(f, "Text"),
            MessageType::Image => write!(f, "Image"),
            MessageType::RichText => write!(f, "RichText"),
        }
    }
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

pub struct MessageInfo {
    pub id: Option<u64>,
    pub kind: String,
    pub content: String,
    pub g_id: u64,
    pub sender_uid: u64,
    pub client_msg_id: String,
    pub create_time: String,
    pub update_time: String,
}

impl MessageInfo {
    pub fn from(msg_body: &MsgBody) -> MessageInfo {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        MessageInfo {
            id: None,
            kind: msg_body.kind.to_string(),
            content: msg_body.content.clone(),
            g_id: msg_body.gid.expect("gid should not be null"),
            sender_uid: msg_body.uid,
            client_msg_id: msg_body.client_msg_id.clone(),
            create_time: now.clone(),
            update_time: now.clone(),
        }
    }
}

pub struct MessageInbox {
    pub id: Option<u64>,
    pub g_id: u64,
    pub m_id: u64,
    pub receiver_uid: u64,
    pub send_status: u8,
    pub read_status: u8,
    pub read_time: String,
    pub create_time: String,
    pub update_time: String,
}
