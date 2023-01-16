use anyhow::Result;

use crate::common::store::STATUS_FALSE;

use super::{friend_model::{FriendAddForm, FriendRelation}, friend_dao::insert_friend_rel};

pub fn add_friend(uid: u64, add_form: &FriendAddForm) -> Result<bool> {
    let status = STATUS_FALSE;
    let friend = FriendRelation::init(uid, add_form.uid, status);
    insert_friend_rel(&friend)
}
