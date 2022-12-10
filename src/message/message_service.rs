use anyhow::{Ok, Result};

use crate::{group::group_dao::select_group_by_ids, user::user_dao::select_user_by_uids};

use super::{
    message_dao::{select_chat_list_page, select_msg_by_ids, select_msg_inbox_for_gu_page},
    message_model::{ChatList, ChatListForm, ChatMessage, MessageForm, MessageInbox},
};

pub fn query_chat_list_page(form: &ChatListForm) -> Result<Vec<ChatMessage>> {
    let chat_list = select_chat_list_page(form);
    if chat_list.is_err() {
        return Ok(Vec::new());
    }
    let list = chat_list.unwrap();
    if list.is_empty() {
        return Ok(Vec::new());
    }

    query_chat_message(list)
}

pub fn query_chat_group_msg_history(form: &MessageForm) -> Result<Vec<ChatMessage>> {
    let msg_inboxs = select_msg_inbox_for_gu_page(form);
    if msg_inboxs.is_err() {
        return Ok(Vec::new());
    }
    let msg_inbox_list = msg_inboxs.unwrap();
    if msg_inbox_list.is_empty() {
        return Ok(Vec::new());
    }
    let chat_list = convert_inbox_to_chat_list(msg_inbox_list);
    query_chat_message(chat_list)
}

fn convert_inbox_to_chat_list(msg_inbox_list: Vec<MessageInbox>) -> Vec<ChatList> {
    msg_inbox_list
        .iter()
        .map(|r| ChatList {
            id: None,
            g_id: r.g_id,
            u_id: r.sender_uid,
            last_msg_id: r.m_id.unwrap(),
            update_time: "".into(),
        })
        .collect::<Vec<ChatList>>()
}

fn query_chat_message(chat_list: Vec<ChatList>) -> Result<Vec<ChatMessage>> {
    let mut gids: Vec<u64> = Vec::new();
    let mut uids: Vec<u64> = Vec::new();
    let mut mids: Vec<u64> = Vec::new();
    for ele in &chat_list {
        gids.push(ele.g_id);
        uids.push(ele.u_id);
        mids.push(ele.last_msg_id);
    }

    let mut result: Vec<ChatMessage> = Vec::new();
    let group_list = select_group_by_ids(gids)?;
    let user_list = select_user_by_uids(uids)?;
    let msg_list = select_msg_by_ids(mids)?;
    for ele in &chat_list {
        let group = group_list
            .iter()
            .find(|r| r.id.unwrap() == ele.g_id)
            .expect("group info not found in chat list");
        let user = user_list
            .iter()
            .find(|r| r.id.unwrap() == ele.u_id)
            .expect("user info not found in chat list");
        let msg = msg_list
            .iter()
            .find(|r| r.id.unwrap() == ele.last_msg_id)
            .expect("msg info not found in chat list");
        let chat_message = ChatMessage::from((*group).clone(), (*msg).clone(), (*user).clone());
        result.push(chat_message);
    }

    result.sort_by(|a,b| a.msg.id.unwrap().cmp(&b.msg.id.unwrap()));

    Ok(result)
}
