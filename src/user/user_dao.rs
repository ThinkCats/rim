use anyhow::{Ok, Result};
use mysql::prelude::Queryable;

use crate::common::store::get_conn;

use super::user_model::User;

type UserRow = (u64, String, String, String, String);

pub fn select_user(uids: Vec<u64>) -> Option<Vec<User>> {
    let uids_join = uids.iter().map(|r| r.to_string()).collect::<Vec<String>>().join(",");
    let sql = format!(
        "select id,name,avatar,email,account from `user` where id in ( {} )",
        uids_join
    );
    println!("SQL:{}", sql);
    let mut conn = get_conn();
    let result: Vec<UserRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
    if result.is_empty() {
        return None;
    }

    let d = result.iter().map(|r|  User {
            id: Some(r.0),
            name: r.1.clone(),
            avatar: r.2.clone(),
            email: Some(r.3.clone()),
            account: r.4.clone(),
            password: Some(String::from("")),
        }).collect::<Vec<User>>();

    Some(d)
}

pub fn insert_user(user: &User) -> Result<u64> {
    let sql = r"insert into `user`(name, avatar,email,account, password) value(?,?,?,?,?)";
    let mut conn = get_conn();
    let _: Vec<u64> = conn
        .exec(
            sql,
            (
                &user.name,
                &user.avatar,
                &user.email,
                &user.account,
                &user.password,
            ),
        )
        .unwrap();
    Ok(conn.last_insert_id())
}

pub fn has_account(account: &String) -> bool {
    let sql = format!("select id from `user` where account = '{}'", account);
    let result: Vec<u64> = get_conn().query(sql).unwrap();
    !result.is_empty()
}
