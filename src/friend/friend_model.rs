use serde::{Deserialize, Serialize};

use crate::common::time::now_time_str;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendAddForm {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendRelation {
    pub id: Option<u64>,
    pub uid: u64,
    pub fid: u64,
    pub status: u8,
    #[serde(rename(serialize = "createTime", deserialize = "createTime"))]
    pub create_time: String,
    #[serde(rename(serialize = "updateTime", deserialize = "updateTime"))]
    pub update_time: String,
}

impl FriendRelation {
    pub fn init(uid: u64, f_uid: u64, status: u8) -> FriendRelation {
        let now = now_time_str();
        FriendRelation {
            id: None,
            uid,
            fid: f_uid,
            status,
            create_time: now.clone(),
            update_time: now.clone(),
        }
    }
}
