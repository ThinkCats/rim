use anyhow::Result;
use mysql::{params, prelude::Queryable, TxOpts};
use serde::{Deserialize, Serialize};

use crate::store::{get_conn, DB_POOL};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupCreateForm {
    pub group: Group,
    pub users: Vec<GroupUser>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: Option<u64>,
    pub name: String,
    pub avatar: String,
    pub mode: u8,
    #[serde(alias = "creatorUid")]
    pub creator_uid: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupUser {
    pub uid: u64,
    pub role: u8,
}

pub fn create_group(form: &GroupCreateForm) -> Result<u64> {
    let mut conn = get_conn();
    let mut tx = conn.start_transaction(TxOpts::default()).unwrap();
    let group_sql = "insert into `groups`(name, avatar,mode,creator_uid) value (?,?,?,?)";
    let _: Vec<u64> = tx
        .exec(
            group_sql,
            (
                &form.group.name,
                &form.group.avatar,
                &form.group.mode,
                &form.group.creator_uid,
            ),
        )
        .unwrap();
    let group_id = tx.last_insert_id().unwrap();
    let group_user_sql = r"insert into group_user(g_id,u_id,role) values(:g_id,:u_id,:role)";
    tx.exec_batch(
        group_user_sql,
        form.users.iter().map(|user| {
            params! {
                "g_id" => group_id,
                "u_id" => user.uid,
                "role" => user.role,
            }
        }),
    ).unwrap();
    tx.commit().unwrap();

    Ok(group_id)
}

type GroupRow = (u64, String, String, u8);

pub fn query_group(id: u64) {
    let sql = format!("select id,name,avatar,type from `groups` where id = {}", id);
    println!("SQL:{}", sql);
    let mut conn = DB_POOL.lock().unwrap().get_conn().unwrap();
    let result: Vec<GroupRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
}
