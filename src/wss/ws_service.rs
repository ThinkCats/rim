use futures::channel::mpsc::UnboundedSender;
use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    message::message_model::{EventType, MsgAck, MsgBody, MsgEvent, MessageInfo},
    user::user_dao::select_user_token_by_token,
    ws::{Sender, UserPeerMap}, group::group_service::user_in_group,
};

pub fn handle_ws_msg(msg: &MsgEvent, user_channel_map: &UserPeerMap, current_sender: &Sender) {
    match msg.event {
        EventType::Login => {
            println!("handle login event");
            handle_login(&msg.body, user_channel_map, current_sender);
        }
        EventType::Msg => {
            println!("handle msg incoming");
            handle_msg(&msg.body, user_channel_map, current_sender);
        }
        EventType::Logout => {}
        EventType::Heart => {}
        EventType::Ack => {}
    }
}

fn handle_login(body: &MsgBody, user_channel_map: &UserPeerMap, current_sender: &Sender) {
    let token = body.content.clone();
    let uid = body.uid;
    let user_token = select_user_token_by_token(token);
    match user_token {
        Some(d) => {
            if uid == d.u_id {
                let s = current_sender.clone();
                send_msg(&s, MsgAck::ack(body.client_msg_id.clone()));
                user_channel_map
                    .lock()
                    .unwrap()
                    .insert(uid, current_sender.clone());
            }
        }
        None => {
            println!("invalid user token, do nothing");
        }
    }
}

fn handle_msg(body: &MsgBody, user_channel_map: &UserPeerMap, current_sender: &Sender) {
    if body.gid.is_none() {
        return;
    }
    let uid = body.uid;
    let gid = body.gid.unwrap();
    if !user_in_group(uid, gid) {
        println!("[warn] handle msg user not in group");
        return;
    }
    //save msg
    save_new_msg(body);

    //send to others

}

fn save_new_msg(body: &MsgBody) {
   let msg_info = MessageInfo::from(body);
   todo!("save msg info and inbox");
}

fn send_msg<T>(sender: &UnboundedSender<Message>, t: T)
where
    T: Serialize,
{
    let msg_str = serde_json::to_string(&t).unwrap();
    sender.unbounded_send(Message::Text(msg_str)).unwrap();
}
