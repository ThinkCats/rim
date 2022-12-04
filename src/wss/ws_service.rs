use anyhow::Result;
use futures::channel::mpsc::UnboundedSender;

use serde::Serialize;
use tokio_tungstenite::tungstenite::Message;

use crate::{
    common::store::STATUS_TRUE,
    group::{group_dao::select_group_user, group_model::GroupUser, group_service::user_in_group},
    message::{
        message_dao::{insert_messages, select_msg_inbox, update_inbox_send_status},
        message_model::{
            EventType, MessageInbox, MessageInfo, MessageType, MsgAck, MsgBody, MsgEvent,
        },
    },
    user::user_dao::select_user_token_by_token,
    ws::{Sender, UserPeerMap},
};

pub fn handle_ws_msg(msg: &MsgEvent, user_channel_map: &UserPeerMap, current_sender: &Sender) {
    match msg.event {
        EventType::Login => {
            println!("handle login event");
            handle_login(&msg.body, user_channel_map, current_sender);
        }
        EventType::Msg => {
            println!("handle msg incoming");
            if valid_user_token(&msg.body, user_channel_map) {
                handle_msg(&msg.body, user_channel_map, current_sender);
            }
        }
        EventType::Logout => {}
        EventType::Heart => {}
        EventType::Ack => {
            println!("handle ack msg");
            //update send status
        }
    }
}

fn handle_login(body: &MsgBody, user_channel_map: &UserPeerMap, current_sender: &Sender) {
    let token = body.content.clone();
    let uid = body.uid;
    let user_token = select_user_token_by_token(token);
    match user_token {
        Some(d) => {
            if uid == d.u_id {
                send_ack(body, None, current_sender);
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

fn send_ack(body: &MsgBody, server_msg_id: Option<u64>, current_sender: &Sender) {
    send_msg(
        current_sender,
        MsgAck::ack(body.client_msg_id.clone(), server_msg_id),
    );
}

fn valid_user_token(body: &MsgBody, user_channel_map: &UserPeerMap) -> bool {
    let sender_uid = body.uid;
    let r = user_channel_map.lock().unwrap();
    let t = r.get(&sender_uid);
    return t.is_some();
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

    let group_user = select_group_user(body.gid.unwrap());
    //save msg
    let msg_id = save_new_msg(body, group_user.clone());
    if msg_id.is_err() {
        println!("[warn]save msg fail");
        return;
    }
    let m_id = msg_id.unwrap();
    //send ack
    send_ack(body, Some(m_id), current_sender);
    //send to others
    send_to_others(m_id, body, group_user.clone(), user_channel_map);
}

fn send_to_others(
    msg_id: u64,
    body: &MsgBody,
    group_user: Vec<GroupUser>,
    user_channel_map: &UserPeerMap,
) {
    let msg = MsgEvent {
        event: EventType::Msg,
        body: MsgBody {
            kind: MessageType::Text,
            uid: body.uid,
            gid: body.gid,
            content: body.content.clone(),
            client_msg_id: body.client_msg_id.clone(),
            msg_id: Some(msg_id),
        },
    };
    for ele in group_user {
        if ele.uid != body.uid {
            let user_channel = user_channel_map.lock().unwrap();
            let sender = user_channel.get(&ele.uid);
            match sender {
                Some(s) => {
                    send_msg(s, msg.clone());
                    update_inbox_send_staus_ok(msg_id, body.gid.unwrap(), ele.uid);
                }
                None => {
                    println!("no reciver found in user channel map");
                }
            }
        }
    }
}

fn save_new_msg(body: &MsgBody, group_user: Vec<GroupUser>) -> Result<u64> {
    let msg_info = MessageInfo::from(body);
    let msg_inboxs = group_user
        .iter()
        .map(|r| MessageInbox::from(body, &msg_info, r.uid, r.uid == body.uid))
        .collect::<Vec<MessageInbox>>();
    let result = insert_messages(&msg_info, msg_inboxs);
    if result.is_err() {
        println!("save new msg error");
    }
    result
}

fn update_inbox_send_staus_ok(msg_id: u64, gid: u64, rev_uid: u64) {
    let msg_inbox = select_msg_inbox(gid, msg_id, rev_uid);
    match msg_inbox {
        Some(inbox) => {
            let _ = update_inbox_send_status(inbox.id.unwrap(), STATUS_TRUE);
        }
        None => {
            println!("[warn] can not find msg inbox when update send status")
        }
    }
}

fn send_msg<T>(sender: &UnboundedSender<Message>, t: T)
where
    T: Serialize,
{
    let msg_str = serde_json::to_string(&t).unwrap();
    println!("send msg :{}", msg_str);
    sender.unbounded_send(Message::Text(msg_str)).unwrap();
}
