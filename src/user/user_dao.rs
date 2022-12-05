use anyhow::{Ok, Result};
use log::info;
use mysql::prelude::Queryable;

use crate::common::store::get_conn;

use super::user_model::{User, UserToken};

type UserRow = (u64, String, String, String, String, String);

pub fn select_user_by_uids(uids: Vec<u64>) -> Result<Vec<User>> {
    let uids_join = uids
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
        .join(",");
    let sql = format!(
        "select id,name,avatar,email,account,'' from `user` where id in ( {} )",
        uids_join
    );
    info!("SQL:{}", sql);
    select_user(sql)
}

pub fn select_user_by_account(account: String) -> Option<User> {
    let sql = format!(
        "select id,name,avatar,email,account,password from `user` where account = '{}'",
        account
    );
    let users = select_user(sql).unwrap();
    if users.is_empty() {
        return None;
    }

    Some(users[0].clone())
}

fn select_user(sql: String) -> Result<Vec<User>> {
    let mut conn = get_conn();
    let result: Vec<UserRow> = conn.query(sql).unwrap();
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

    Ok(d)
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

type UserTokenRow = (u64, u64, String, String);
pub fn select_user_token_by_uid(uid: u64) -> Option<UserToken> {
    let sql = format!(
        "select id,u_id,token,expire_time from `user_token` where u_id= '{}'",
        uid
    );
    let user_token = select_user_token(sql);
    match user_token {
        Some(d) => Some(d[0].clone()),
        None => None,
    }
}

pub fn select_user_token_by_token(token: String) -> Option<UserToken> {
    let sql = format!(
        "select id,u_id,token,expire_time from `user_token` where token= '{}'",
        token
    );
    let user_token = select_user_token(sql);
    match user_token {
        Some(d) => Some(d[0].clone()),
        None => None,
    }
}

fn select_user_token(sql: String) -> Option<Vec<UserToken>> {
    let result: Vec<UserTokenRow> = get_conn().query(sql).expect("query token error");
    if result.is_empty() {
        return None;
    }
    let d = result
        .iter()
        .map(|r| UserToken {
            id: Some(r.0),
            u_id: r.1,
            token: r.2.clone(),
            expire_time: r.3.clone(),
        })
        .collect::<Vec<UserToken>>();
    return Some(d);
}

pub fn insert_token(token: &UserToken) -> Result<u64> {
    let sql = r"insert into user_token(u_id,token,expire_time) value(?,?,?)";
    let mut conn = get_conn();
    let _: Vec<u64> = conn
        .exec(sql, (&token.u_id, &token.token, &token.expire_time))
        .expect("save token error");
    Ok(conn.last_insert_id())
}

pub fn update_token(token: &UserToken) -> Result<u64> {
    let sql = r"update user_token set token = ?, expire_time = ? where id = ?";
    let mut conn = get_conn();
    let _: Vec<u64> = conn
        .exec(sql, (&token.token, &token.expire_time, &token.id.unwrap()))
        .expect("save token error");
    Ok(conn.last_insert_id())
}
