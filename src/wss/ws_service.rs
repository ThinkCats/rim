use crate::{
    message::message_model::{EventType, MsgBody, MsgEvent},
    ws::{Sender, UserPeerMap}, user::user_dao::select_user_token_by_token,
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
        Some(u_t) => {
            if uid == u_t.u_id {
                user_channel.lock().unwrap().insert(uid, sender.clone());
            }
        },
        None => {
            println!("invalid user token, do nothing");
        },
    }
}
