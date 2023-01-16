use anyhow::Result;
use mysql::prelude::Queryable;

use crate::common::store::get_conn;

use super::friend_model::FriendRelation;

pub fn insert_friend_rel(friend: &FriendRelation) -> Result<bool> {
    let mut conn = get_conn();
    let sql = "insert into friend_rel(uid,f_id,status) values (?,?,?)";
    let _: Vec<u64> = conn
        .exec(sql, (&friend.uid, &friend.fid, &friend.status))
        .expect("insert friend rel error");
    Ok(true)
}
