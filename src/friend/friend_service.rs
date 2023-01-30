use anyhow::Result;

use crate::user::user_model::User;

use super::{
    friend_dao::insert_friend_rel,
    friend_model::{FriendAddForm, FriendRelation, FriendStatusModifyForm, FRIEND_STATUS_APPLY},
};

pub fn add_friend(add_form: &FriendAddForm) -> Result<bool> {
    let status = FRIEND_STATUS_APPLY;
    let friend = FriendRelation::init(add_form.uid.unwrap(), add_form.fid, status);
    insert_friend_rel(&friend)
}

pub fn list_friend(uid: u64) -> Result<Vec<User>> {
    todo!()
}

pub fn modify_friend_status(modify_form: &FriendStatusModifyForm) -> Result<bool> {
    todo!()
}
