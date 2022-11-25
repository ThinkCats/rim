use anyhow::{Ok, Result};
use mysql::prelude::Queryable;

use crate::common::store::get_conn;

use super::user_model::{User, UserToken};

type UserRow = (u64, String, String, String, String, String);

pub fn select_user_by_uids(uids: Vec<u64>) -> Option<Vec<User>> {
    let uids_join = uids
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let sql = format!(
        "select id,name,avatar,email,account,'' from `user` where id in ( {} )",
        uids_join
    );
    println!("SQL:{}", sql);
    select_user(sql)
}

pub fn select_user_by_account(account: String) -> Option<User> {
    let sql = format!(
        "select id,name,avatar,email,account,password from `user` where account = '{}'",
        account
    );
    let users = select_user(sql);
    users.and_then(|r| Some(r[0].clone()))
}

fn select_user(sql: String) -> Option<Vec<User>> {
    let mut conn = get_conn();
    let result: Vec<UserRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
    if result.is_empty() {
        return None;
    }

    let d = result
        .iter()
        .map(|r| User {
            id: Some(r.0),
            name: r.1.clone(),
            avatar: r.2.clone(),
            email: Some(r.3.clone()),
            account: r.4.clone(),
            password: Some(r.5.clone()),
        })
        .collect::<Vec<User>>();

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

// pub fn select_user_token(uid: u64) -> Option<UserToken> {
//     let sql = format!("select id from `user` where account = '{}'", account);
   
// }

pub fn insert_token(token: &UserToken) -> Result<u64> {
    let sql = r"insert into user_token(u_id,token,expire_time) value(?,?,?)";
    let mut conn = get_conn();
    let _: Vec<u64> = conn
        .exec(sql, (&token.u_id, &token.token, &token.expire_time))
        .expect("save token error");
    Ok(conn.last_insert_id())
}
