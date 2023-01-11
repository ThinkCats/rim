use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendAddForm {
    pub uid: u64,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct FriendRelation {
    pub id: u64,
    pub uid: u64,
    pub fid: u64,
    pub status: u8,
}
