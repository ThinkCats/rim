use std::fmt::Display;

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{
    common::store::{STATUS_FALSE, STATUS_TRUE},
    group::group_model::Group,
    user::user_model::User,
};

#[derive(Serialize, Deserialize, Clone)]
pub enum EventType {
    Login,
    Msg,
    Read,
    Heart,
    Logout,
    Ack,
    Notify,
}

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct MsgBody {
    pub kind: MessageType,
    pub uid: u64,
    pub gid: Option<u64>,
    pub content: String,
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
    #[serde(rename(serialize = "msgId", deserialize = "msgId"))]
    pub msg_id: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MsgEvent {
    pub event: EventType,
    pub body: MsgBody,
}

#[derive(Serialize, Deserialize)]
pub struct MsgAck {
    #[serde(rename(serialize = "clientMsgId", deserialize = "clientMsgId"))]
    pub client_msg_id: String,
    #[serde(rename(serialize = "serverMsgId", deserialize = "serverMsgId"))]
    pub server_msg_id: Option<u64>,
    pub kind: EventType,
    pub content: String,
}

impl MsgAck {
    pub fn ack(client_msg_id: String, server_msg_id: Option<u64>) -> MsgAck {
        MsgAck {
            client_msg_id,
            server_msg_id,
            kind: EventType::Ack,
            content: "Ok".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageInfo {
    pub id: Option<u64>,
    pub kind: String,
    pub content: String,
    #[serde(rename(serialize = "gid", deserialize = "gid"))]
    pub g_id: u64,
    #[serde(rename(serialize = "uid", deserialize = "uid"))]
    pub sender_uid: u64,
    pub client_msg_id: String,
    #[serde(rename(serialize = "createTime", deserialize = "createTime"))]
    pub create_time: String,
    #[serde(rename(serialize = "updateTime", deserialize = "updateTime"))]
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

#[derive(Serialize, Deserialize, Clone)]
pub struct MessageInbox {
    pub id: Option<u64>,
    pub g_id: u64,
    pub m_id: Option<u64>,
    pub receiver_uid: u64,
    pub sender_uid: u64,
    pub send_status: u8,
    pub read_status: u8,
    pub read_time: Option<String>,
    pub create_time: String,
    pub update_time: String,
}

impl MessageInbox {
    pub fn from(
        msg_body: &MsgBody,
        msg_info: &MessageInfo,
        receiver_uid: u64,
        sender_uid: u64,
        self_receiver: bool,
    ) -> MessageInbox {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        MessageInbox {
            id: None,
            g_id: msg_body.gid.expect("gid should not be null"),
            m_id: msg_info.id,
            receiver_uid,
            sender_uid,
            send_status: if self_receiver {
                STATUS_TRUE
            } else {
                STATUS_FALSE
            },
            read_status: if self_receiver {
                STATUS_TRUE
            } else {
                STATUS_FALSE
            },
            read_time: None,
            create_time: now.clone(),
            update_time: now.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatList {
    pub id: Option<u64>,
    pub g_id: u64,
    pub u_id: u64,
    pub last_msg_id: u64,
    pub chat_uid: u64,
    pub update_time: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub group: Group,
    pub msg: MessageInfo,
    pub user: User,
    pub unread: Option<u32>,
}

impl ChatMessage {
    pub fn from(group: Group, msg: MessageInfo, user: User, unread: Option<u32>) -> ChatMessage {
        ChatMessage {
            group,
            msg,
            user,
            unread,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserGroupUnread {
    pub gid: u64,
    pub uid: u64,
    pub unread: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ChatListForm {
    pub uid: u64,
    pub page: u32,
    pub size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct MessageForm {
    pub gid: u64,
    pub uid: u64,
    pub page: u32,
    pub size: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ChatGroupReadForm {
    pub gid: u64,
    pub uid: u64, 
}