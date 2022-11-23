use anyhow::{Result, bail, Ok};
use mysql::prelude::Queryable;

use serde::{Deserialize, Serialize};

use crate::store::get_conn;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<u64>,
    pub name: String,
    pub avatar: String,
    pub email: Option<String>,
    pub account: String,
    pub password: Option<String>,
}

type UserRow = (u64, String, String, String, String);

pub fn query_user(uid: u64) -> Option<User> {
    let sql = format!(
        "select id,name,avatar,email,account from `user` where id = {}",
        uid
    );
    println!("SQL:{}", sql);
    let mut conn = get_conn();
    let result: Vec<UserRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
    if result.is_empty() {
        return None;
    }
    let r = result.get(0).unwrap();
    let user = User {
        id: Some(r.0),
        name: r.1.clone(),
        avatar: r.2.clone(),
        email: Some(r.3.clone()),
        account: r.4.clone(),
        password: Some(String::from("")),
    };
    Some(user)
}

pub fn create_user(user: &User) -> Result<u64> {
    if has_account(&user.account) {
        bail!("Account Existed")
    }

    let sql = r"insert into `user`(name, avatar,email,account, password) value(?,?,?,?,?)";
    let mut conn = get_conn();
    let _:Vec<u64> = conn.exec(
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
    println!("Execute Result:{:?}",conn.last_insert_id());
    Ok(conn.last_insert_id())
}

fn has_account(account: &String) -> bool {
    let sql = format!("select id from `user` where account = '{}'",account);
    println!("Sql:{}",sql);
    let result:Vec<u64> = get_conn().query(sql).unwrap();
    !result.is_empty()
}

