use serde::{Deserialize, Serialize};

use crate::user::user_model::User;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupCreateForm {
    pub group: Group,
    pub users: Vec<GroupUserForm>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupUserForm {
    pub uid: u64,
    pub role: u8,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Group {
    pub id: Option<u64>,
    pub name: String,
    pub avatar: String,
    pub mode: u8,
    #[serde(rename(serialize = "creatorUid", deserialize = "creatorUid"))]
    pub creator_uid: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupUser {
    pub role: u8,
    pub uid: u64,
    pub gid: u64,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct GroupUserDTS {
    pub role: u8,
    pub user: User,
}
