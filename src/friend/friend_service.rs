use anyhow::{Result, Ok};
use rocket::response::status;

use crate::user::{user_dao::select_user_by_uids, user_model::User};

use super::{
    friend_dao::{insert_friend_rel, select_friend},
    friend_model::{
        FriendAddForm, FriendQueryForm, FriendRelation, FriendStatusModifyForm, FRIEND_STATUS_APPLY,
    },
};

pub fn add_friend(add_form: &FriendAddForm) -> Result<bool> {
    let status = FRIEND_STATUS_APPLY;
    let friend = FriendRelation::init(add_form.uid.unwrap(), add_form.fid, status);
    insert_friend_rel(&friend)
}

pub fn list_friend(uid: u64, status: u8) -> Result<Vec<User>> {
    let query = FriendQueryForm {
        uid,
        status,
        page: None,
        size: None,
    };
    let friend_rels = select_friend(&query)?;
    if friend_rels.is_empty() {
        return Ok(Vec::new());
    }
    let uids = friend_rels.iter().map(|r| r.fid).collect();
    select_user_by_uids(uids)
}

pub fn modify_friend_status(modify_form: &FriendStatusModifyForm) -> Result<bool> {
    todo!()
}
