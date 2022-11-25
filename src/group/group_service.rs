use anyhow::Result;


use crate::user::user_dao::select_user_by_uids;

use super::{
    group_dao::{insert_group, select_group, select_group_user},
    group_model::{Group, GroupCreateForm, GroupUserDTS},
};

pub fn create_group(form: &GroupCreateForm) -> Result<u64> {
    insert_group(form)
}

pub fn query_group(uid: u64) -> Result<Vec<Group>> {
    select_group(uid)
}

pub fn query_group_user(gid: u64) -> Result<Vec<GroupUserDTS>> {
    let group_users = select_group_user(gid);
    if group_users.is_empty() {
        return Ok(Vec::new());
    }

    let uids = group_users.iter().map(|d| d.uid).collect::<Vec<u64>>();
    let users = select_user_by_uids(uids);
    if users.is_none() {
        return Ok(Vec::new());
    }

    let uu = users.unwrap();
    let result = group_users
        .iter()
        .map(|r| {
            let user = &uu
                .iter()
                .filter(|u| r.uid == u.id.unwrap())
                .next()
                .expect("no user found ");
            GroupUserDTS {
                role: r.role,
                user: (*user).clone(),
            }
        })
        .collect::<Vec<GroupUserDTS>>();

    Ok(result)
}
