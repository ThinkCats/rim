use std::sync::Mutex;

use lazy_static::lazy_static;
use mysql::{Pool, prelude::Queryable};

lazy_static! {
    pub static ref MYSQL_URL:String = String::from("mysql://root:12345678@localhost:3306/rim");
    pub static ref DB_POOL: Mutex<Pool> = Mutex::new(Pool::new(MYSQL_URL.as_str()).unwrap());
}


type GroupRow = (u64,String,String,u8);

pub fn query_group(id: u64) {
    let sql = format!( "select id,name,avatar,type from `groups` where id = {}",id); 
    println!("SQL:{}",sql);
    let mut conn = DB_POOL.lock().unwrap().get_conn().unwrap();
    let result: Vec<GroupRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
}