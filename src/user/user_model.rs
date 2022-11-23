use serde::{Deserialize, Serialize};

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
