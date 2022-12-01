use futures::channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    message::message_model::{EventType, MsgBody, MsgEvent, MsgAck},
    user::user_dao::select_user_token_by_token,
    ws::{Sender, UserPeerMap},
};

pub fn handle_ws_msg(msg: &MsgEvent, user_channel: &UserPeerMap, sender: &Sender) {
    match msg.event {
        EventType::Login => {
            println!("handle login event");
            handle_login(&msg.body, user_channel, sender);
        }
        EventType::Msg => {}
        EventType::Logout => {}
        EventType::Heart => {}
        _ => {
            println!("unknown event")
        }
    }
}

fn handle_login(body: &MsgBody, user_channel: &UserPeerMap, sender: &Sender) {
    let token = body.content.clone();
    let uid = body.uid;
    let user_token = select_user_token_by_token(token);
    match user_token {
        Some(d) => {
            if uid == d.u_id {
                let s = sender.clone();
                send_msg(&s,  MsgAck::ack(body.client_msg_id.clone()));
                user_channel.lock().unwrap().insert(uid, sender.clone());
            }
        }
        None => {
            println!("invalid user token, do nothing");
        }
    }
}

fn send_msg<T>(sender: &UnboundedSender<Message>, t: T)
where
    T: Serialize,
{
    let msg_str = serde_json::to_string(&t).unwrap();
    sender.unbounded_send(Message::Text(msg_str)).unwrap();
}
