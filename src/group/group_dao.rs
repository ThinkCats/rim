use anyhow::{Ok, Result};
use mysql::{params, prelude::Queryable, TxOpts};

use crate::common::store::get_conn;

use super::group_model::{Group, GroupCreateForm, GroupUpdateForm, GroupUser};

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

pub fn update_group(form: &GroupUpdateForm) -> Result<bool> {
    let sql = "update `groups` set name = ?,  avatar = ? where id = ?";
    let _ = get_conn()
        .exec_drop(sql, (&form.name, &form.avatar, &form.id))
        .expect("update group error");
    Ok(true)
}

pub fn select_group_by_ids(ids: Vec<u64>) -> Result<Vec<Group>> {
    let ids_join = ids
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let sql = format!(
        "select id,name,avatar,mode,creator_uid from `groups` where id in ({})",
        ids_join
    );
    select_group(sql)
}

pub fn select_group_by_u(uid: u64) -> Result<Vec<Group>> {
    let sql = format!(
        "select id,name,avatar,mode,creator_uid from `groups` where id in
         (select g_id from group_user where u_id = {})",
        uid
    );
    select_group(sql)
}

type GroupRow = (u64, String, String, u8, u64);
fn select_group(sql: String) -> Result<Vec<Group>> {
    let mut conn = get_conn();
    let result: Vec<GroupRow> = conn.query(sql).expect("query data error");

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

type GroupUserRow = (u64, u64, u8, u64);
pub fn select_group_user(gid: u64) -> Vec<GroupUser> {
    let sql = format!(
        "select g_id,u_id,role,id from group_user where g_id = {}",
        gid
    );
    let result: Vec<GroupUserRow> = get_conn().query(sql).expect("query data error");
    result
        .iter()
        .map(|r| GroupUser {
            gid: r.0,
            uid: r.1,
            role: r.2,
            id: Some(r.3),
        })
        .collect::<Vec<GroupUser>>()
}

pub fn insert_group_user(group_users: Vec<GroupUser>) -> Result<u64> {
    let values = group_users
        .iter()
        .map(|r| format!("({},{},{})", r.gid, r.uid, r.role))
        .collect::<Vec<String>>()
        .join(",");
    let group_user_sql = format!("insert into group_user(g_id,u_id,role) values {}", values);
    let mut conn = get_conn();
    conn.query_drop(group_user_sql)
        .expect("add group user error");
    Ok(conn.last_insert_id())
}

pub fn delete_group_user(group_users: Vec<GroupUser>) -> Result<bool> {
    let ids = group_users
        .iter()
        .map(|r| r.id.unwrap().to_string())
        .collect::<Vec<String>>()
        .join(",");
    let del_sql = format!("delete from group_user where id in ({})", ids);
    get_conn()
        .query_drop(del_sql)
        .expect("delete group user error");
    Ok(true)
}
