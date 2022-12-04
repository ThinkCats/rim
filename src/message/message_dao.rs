use anyhow::{Ok, Result};
use chrono::NaiveDateTime;
use mysql::{prelude::Queryable, TxOpts};

use crate::common::{store::get_conn, time::format_time};

use super::message_model::{MessageInbox, MessageInfo};

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
    tx.commit().expect("commit message tx error");

    Ok(msg_id)
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
pub fn select_msg_inbox(gid: u64, mid: u64, ruid: u64) -> Option<MessageInbox> {
    let sql = format!(
        "select id,g_id,m_id,receiver_uid,send_status,read_status,read_time,create_time,update_time
         from message_inbox where g_id = {} and m_id = {} and receiver_uid = {}",
        gid, mid, ruid
    );
    let result: Vec<MsgInboxRow> = get_conn().query(sql).expect("query msg inbox error");
    let result = result
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

    if result.is_empty() {
        return None;
    }
    Some(result[0].clone())
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