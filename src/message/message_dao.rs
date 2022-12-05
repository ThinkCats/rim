use anyhow::{Ok, Result};
use chrono::{Local, NaiveDateTime};
use mysql::{prelude::Queryable, Transaction, TxOpts};

use crate::common::{store::get_conn, time::format_time};

use super::message_model::{ChatList, ChatListForm, MessageInbox, MessageInfo};

pub fn insert_messages(msg_info: &MessageInfo, msg_inboxs: Vec<MessageInbox>) -> Result<u64> {
    let mut conn = get_conn();
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    let msg_info_sql =
        "insert into message(kind,content,g_id,sender_uid,client_msg_id) values(?,?,?,?,?)";
    let _: Vec<u64> = tx
        .exec(
            msg_info_sql,
            (
                &msg_info.kind,
                &msg_info.content,
                &msg_info.g_id,
                &msg_info.sender_uid,
                &msg_info.client_msg_id,
            ),
        )
        .expect("insert message info error");
    let msg_id = tx.last_insert_id().unwrap();
    let msg_inbox_sql = "insert into message_inbox(g_id,m_id,receiver_uid,send_status,read_status) values(?,?,?,?,?)";
    tx.exec_batch(
        msg_inbox_sql,
        msg_inboxs.iter().map(|r| {
            (
                &r.g_id,
                &msg_id,
                &r.receiver_uid,
                &r.send_status,
                &r.read_status,
            )
        }),
    )
    .expect("insert message inbox error");

    for ele in msg_inboxs {
        let chat_list = select_chat_list_by_gu(ele.g_id, ele.receiver_uid);
        match chat_list {
            Some(d) => {
                //update
                let _ = update_chat_list(&mut tx, d.id.unwrap(), msg_id);
            }
            None => {
                //insert
                let list = ChatList {
                    id: None,
                    g_id: ele.g_id,
                    u_id: ele.receiver_uid,
                    last_msg_id: msg_id,
                    update_time: format_time(Local::now().naive_local()),
                };
                let _ = insert_chat_list_with_trans(&mut tx, list);
            }
        }
    }

    tx.commit().expect("commit message tx error");

    Ok(msg_id)
}

type MessageRow = (
    u64,
    String,
    String,
    u64,
    u64,
    String,
    NaiveDateTime,
    NaiveDateTime,
);
pub fn select_msg_by_ids(ids: Vec<u64>) -> Result<Vec<MessageInfo>> {
    let ids_join = ids
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let sql = format!(
        "select id,kind,content,g_id,sender_uid,client_msg_id,create_time,update_time from message
         where id in ({})",
        ids_join
    );
    let result: Vec<MessageRow> = get_conn().query(sql).expect("select message error");
    let d = result
        .iter()
        .map(|r| MessageInfo {
            id: Some(r.0),
            kind: r.1.clone(),
            content: r.2.clone(),
            g_id: r.3,
            sender_uid: r.4,
            client_msg_id: r.5.clone(),
            create_time: format_time(r.6),
            update_time: format_time(r.7),
        })
        .collect::<Vec<MessageInfo>>();
    Ok(d)
}

fn select_chat_list_by_gu(gid: u64, uid: u64) -> Option<ChatList> {
    let sql = format!(
        "select id,g_id,u_id,last_msg_id,update_time from chat_list where g_id={} and u_id ={} order by update_time desc",
        gid, uid
    );

    let list = select_chat_list(sql).unwrap();
    if list.is_empty() {
        return None;
    }
    return Some(list[0].clone());
}

pub fn select_chat_list_page(query: &ChatListForm) -> Result<Vec<ChatList>> {
    let start_idx = (query.page - 1) * query.size;
    let sql = format!(
        "select id,g_id,u_id,last_msg_id,update_time  from chat_list where u_id={}
                      order by update_time desc limit {},{}",
        query.uid, start_idx, query.size
    );
    select_chat_list(sql)
}

type ChatListRow = (u64, u64, u64, u64, NaiveDateTime);
fn select_chat_list(sql: String) -> Result<Vec<ChatList>> {
    let result: Vec<ChatListRow> = get_conn().query(sql).expect("query chat list error");
    let r = result
        .iter()
        .map(|r| ChatList {
            id: Some(r.0),
            g_id: r.1,
            u_id: r.2,
            last_msg_id: r.3,
            update_time: format_time(r.4),
        })
        .collect::<Vec<ChatList>>();
    Ok(r)
}

fn insert_chat_list_with_trans(tx: &mut Transaction, chat_list: ChatList) -> Result<u64> {
    let sql = "insert into chat_list(g_id,u_id,last_msg_id,update_time) values (?,?,?,?)";
    let _: Vec<u64> = tx
        .exec(
            sql,
            (
                &chat_list.g_id,
                &chat_list.u_id,
                &chat_list.last_msg_id,
                &chat_list.update_time,
            ),
        )
        .expect("insert chat list error");
    Ok(tx.last_insert_id().unwrap())
}

fn update_chat_list(tx: &mut Transaction, id: u64, last_msg_id: u64) -> Result<bool> {
    let sql = "update chat_list set last_msg_id = ? where id = ?";
    let _: Vec<u64> = tx
        .exec(sql, (&last_msg_id, &id))
        .expect("update chat list error");

    Ok(true)
}

type MsgInboxRow = (
    u64,
    u64,
    u64,
    u64,
    u8,
    u8,
    Option<NaiveDateTime>,
    NaiveDateTime,
    NaiveDateTime,
);
pub fn select_msg_inbox_for_gmr(gid: u64, mid: u64, ruid: u64) -> Option<MessageInbox> {
    let sql = format!(
        "select id,g_id,m_id,receiver_uid,send_status,read_status,read_time,create_time,update_time
         from message_inbox where g_id = {} and m_id = {} and receiver_uid = {}",
        gid, mid, ruid
    );
    let result = select_msg_inbox(sql);
    if result.is_err() {
        return None;
    }
    let datas = result.unwrap();
    if datas.is_empty() {
        return None;
    }
    Some(datas[0].clone())
}

pub fn select_msg_inbox_for_user(
    ruid: u64,
    page: u32,
    page_size: u32,
) -> Result<Vec<MessageInbox>> {
    let start_idx = (page - 1) * page_size;
    let sql = format!(
        "select id,g_id,m_id,receiver_uid,send_status,read_status,read_time,create_time,update_time
         from message_inbox where  receiver_uid = {} order by id desc limit {},{}",
        ruid, start_idx, page_size
    );
    select_msg_inbox(sql)
}

fn select_msg_inbox(sql: String) -> Result<Vec<MessageInbox>> {
    let result: Vec<MsgInboxRow> = get_conn().query(sql).expect("query msg inbox error");
    let data = result
        .iter()
        .map(|d| MessageInbox {
            id: Some(d.0),
            g_id: d.1,
            m_id: Some(d.2),
            receiver_uid: d.3,
            send_status: d.4,
            read_status: d.5,
            read_time: if d.6.is_some() {
                Some(format_time(d.6.unwrap()))
            } else {
                None
            },
            create_time: format_time(d.7),
            update_time: format_time(d.8),
        })
        .collect::<Vec<MessageInbox>>();
    Ok(data)
}

pub fn update_inbox_send_status(id: u64, send_status: u8) -> Result<bool> {
    let sql = "update message_inbox set send_status = ? where id= ?";
    let _: Vec<u64> = get_conn()
        .exec(sql, (&send_status, &id))
        .expect("update send status error");
    Ok(true)
}

pub fn update_inbox_read_status(id: u64, read_status: u8, read_time: String) -> Result<bool> {
    let sql = "update message_inbox set read_status = ?, read_time = ? where id= ?";
    let _: Vec<u64> = get_conn()
        .exec(sql, (&read_status, &read_time, &id))
        .expect("update send status error");
    Ok(true)
}
