use anyhow::Result;
use mysql::prelude::Queryable;

use crate::{store::DB_POOL, user::User};

pub struct GroupCreateForm {
    pub group: Group,
    pub users: Vec<User>,
}

pub struct Group {
    pub id: Option<u64>,
    pub name: String,
    pub avatar: String,
    pub mode: u8,
    pub creator_uid: u64,
}


pub fn create_group(form: &GroupCreateForm) -> Result<u64> {
    //TODO transcational
    Ok(1)
}

type GroupRow = (u64,String,String,u8);

pub fn query_group(id: u64) {
    let sql = format!( "select id,name,avatar,type from `groups` where id = {}",id); 
    println!("SQL:{}",sql);
    let mut conn = DB_POOL.lock().unwrap().get_conn().unwrap();
    let result: Vec<GroupRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
}