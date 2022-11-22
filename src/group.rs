use anyhow::Result;
use mysql::{params, prelude::Queryable, TxOpts};
use serde::{Deserialize, Serialize};

use crate::store::get_conn;

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
    #[serde(rename(serialize = "creatorUid", deserialize = "creatorUid"))]
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
    )
    .expect("add user error");
    tx.commit().expect("add user tx error");

    Ok(group_id)
}

type GroupRow = (u64, String, String, u8, u64);

pub fn query_group(uid: u64) -> Result<Vec<Group>> {
    let sql = format!(
        "select id,name,avatar,mode,creator_uid from `groups` where id = (select g_id from group_user where u_id = {})",
        uid
    );
    println!("SQL:{}", sql);
    let mut conn = get_conn();
    let result: Vec<GroupRow> = conn.query(sql).expect("query data error");
    println!("result:{:?}", result);

    let d = result.iter().map(|r| {
            Group {
                id: Some(r.0),
                name: r.1.clone(),
                avatar: r.2.clone(),
                mode: r.3,
                creator_uid: r.4,
            }
        }).collect::<Vec<Group>>();

    Ok(d)
}
