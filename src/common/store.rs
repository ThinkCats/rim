use std::{cell::RefCell, sync::Mutex};

use lazy_static::lazy_static;
use mysql::{Pool, PooledConn};

pub const GROUP_USER_OP_TYPE_ADD: u8 = 1;
pub const GROUP_USER_OP_TYPE_REMOVE: u8 = 2;
pub const GROUP_USER_ROLE_COMMON: u8 = 1;
pub const GROUP_USER_ROLE_ADMIN: u8 = 2;

pub const STATUS_TRUE: u8 = 1;
pub const STATUS_FALSE: u8 = 2;


lazy_static! {
    pub static ref MYSQL_URL: String = String::from("mysql://root:12345678@localhost:3306/rim");
    pub static ref DB_POOL: Mutex<Pool> = Mutex::new(Pool::new(MYSQL_URL.as_str()).unwrap());
}

pub fn get_conn() -> PooledConn {
    DB_POOL.lock().unwrap().get_conn().unwrap()
}

#[derive(Debug)]
pub struct ThreadLocalStore {
    pub token: String,
    pub uid: u64,
}

pub fn add_local_store(token: String, uid: u64) {
    THREAD_LOCAL.with(|r| {
        let mut d = r.borrow_mut();
        let token_existed = d.iter().any(|t| t.token == token);
        if !token_existed {
            d.push(ThreadLocalStore { token, uid });
        }
    });
}

thread_local! {
   pub static THREAD_LOCAL: RefCell<Vec<ThreadLocalStore>> = RefCell::new(Vec::new());
}
