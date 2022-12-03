use anyhow::{Ok, Result};
use mysql::{prelude::Queryable, TxOpts};

use crate::common::store::get_conn;

use super::message_model::{MessageInbox, MessageInfo};

pub fn insert_messages(msg_info: &MessageInfo, msg_inboxs: Vec<MessageInbox>) -> Result<bool> {
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

    Ok(true)
}
