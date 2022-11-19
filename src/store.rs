use std::sync::Mutex;

use lazy_static::lazy_static;
use mysql::{Pool, prelude::Queryable};

lazy_static! {
    pub static ref MYSQL_URL:String = String::from("mysql://root:12345678@localhost:3306/rim");
    pub static ref DB_POOL: Mutex<Pool> = Mutex::new(Pool::new(MYSQL_URL.as_str()).unwrap());
}


