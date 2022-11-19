use mysql::prelude::Queryable;

use rocket::serde::Serialize;

use crate::store::DB_POOL;


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: u64,
    pub name: String,
    pub avatar: String,
    pub email: String,
    pub account: String,
    pub password: String,
}

type UserRow = (u64, String, String, String, String);

pub fn query_user(uid: u64) -> Option<User> {
    let sql = format!(
        "select id,name,avatar,email,account from `user` where id = {}",
        uid
    );
    println!("SQL:{}", sql);
    let mut conn = DB_POOL.lock().unwrap().get_conn().unwrap();
    let result: Vec<UserRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
    if result.is_empty() {
        return None;
    }
    let r = result.get(0).unwrap();
    let user = User {
        id: r.0,
        name: r.1.clone(),
        avatar: r.2.clone(),
        email: r.3.clone(),
        account: r.4.clone(),
        password: String::from(""),
    };
    Some(user)
}

pub fn create_user(user: &User) {}
