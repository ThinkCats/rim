use serde::{Deserialize, Serialize};

use crate::common::time::now_time_str;

pub const FRIEND_STATUS_APPLY: u8 = 0;
pub const FRIEND_STATUS_CONFIRM: u8 = 1;
pub const FRIEND_STATUS_REJECT: u8 = 2;

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendAddForm {
    pub fid: u64,
    pub uid: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendStatusModifyForm {
    pub uid: u64,
    pub status: u8,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendQueryForm {
    pub uid: u64,
    pub page: Option<u32>,
    pub size: Option<u32>,
}

impl FriendQueryForm {
    pub fn idx(&self) -> u32 {
        (self.page.unwrap() - 1) * self.size.unwrap()
    }
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
    pub fn init(uid: u64, fid: u64, status: u8) -> FriendRelation {
        let now = now_time_str();
        FriendRelation {
            id: None,
            uid,
            fid,
            status,
            create_time: now.clone(),
            update_time: now.clone(),
        }
    }
}
