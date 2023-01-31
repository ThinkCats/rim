use anyhow::{Ok, Result};
use chrono::NaiveDateTime;
use mysql::prelude::Queryable;

use crate::common::{store::get_conn, time::format_time};

use super::friend_model::{FriendQueryForm, FriendRelation};

pub fn insert_friend_rel(friend: &FriendRelation) -> Result<bool> {
    let mut conn = get_conn();
    let sql = "insert into friend_rel(uid,f_id,status) values (?,?,?)";
    let _: Vec<u64> = conn
        .exec(sql, (&friend.uid, &friend.fid, &friend.status))
        .expect("insert friend rel error");
    Ok(true)
}

type FriendRow = (u64, u64, u64, u8, NaiveDateTime, NaiveDateTime);
pub fn select_friend(query: &FriendQueryForm) -> Result<Vec<FriendRelation>> {
    //TODO query all, not page
    let sql = format!("select id, uid,f_id,status,create_time,update_time from friend_rel where uid = {} order by id asc", query.uid);
    let result: Vec<FriendRow> = get_conn().query(sql).expect("select friend error");
    let r = result
        .iter()
        .map(|r| FriendRelation {
            id: Some(r.0),
            uid: r.1,
            fid: r.2,
            status: r.3,
            create_time: format_time(r.4),
            update_time: format_time(r.5),
        })
        .collect::<Vec<FriendRelation>>();

    Ok(r)
}
