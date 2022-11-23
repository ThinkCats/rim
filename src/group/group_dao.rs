use anyhow::{Ok, Result};
use mysql::{params, prelude::Queryable, TxOpts};

use crate::common::store::get_conn;

use super::group_model::{Group, GroupCreateForm, GroupUser};

pub fn insert_group(form: &GroupCreateForm) -> Result<u64> {
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

pub fn select_group(uid: u64) -> Result<Vec<Group>> {
    let sql = format!(
        "select id,name,avatar,mode,creator_uid from `groups` where id in
         (select g_id from group_user where u_id = {})",
        uid
    );
    let mut conn = get_conn();
    let result: Vec<GroupRow> = conn.query(sql).expect("query data error");
    println!("result:{:?}", result);

    let d = result
        .iter()
        .map(|r| Group {
            id: Some(r.0),
            name: r.1.clone(),
            avatar: r.2.clone(),
            mode: r.3,
            creator_uid: r.4,
        })
        .collect::<Vec<Group>>();

    Ok(d)
}

type GroupUserRow = (u64, u64, u8);
pub fn query_group_user(gid: u64) -> Result<Option<Vec<GroupUser>>> {
    let sql = format!("select g_id,u_id,role from group_user where g_id = {}", gid);
    let result: Vec<GroupUserRow> = get_conn().query(sql).expect("query data error");
    if result.is_empty() {
        return Ok(None);
    }

    let group_users: Vec<GroupUser>;
    for ele in result {
        //TODO Query User
        // group_users.push(value);
    }
    Ok(None)
}
