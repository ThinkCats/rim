use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: Option<u64>,
    pub name: String,
    pub avatar: String,
    pub email: Option<String>,
    pub account: String,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserLoginForm {
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserToken {
    pub id: Option<u64>,
    pub u_id: u64,
    pub token: String,
    pub expire_time: String,
}

impl UserToken {
    pub fn init(u_id: u64, token: String, expire_time: String) -> UserToken {
        UserToken {
            id: None,
            u_id,
            token,
            expire_time,
        }
    }
}
