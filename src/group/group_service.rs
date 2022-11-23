use anyhow::Result;

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

// pub fn query_group_user(gid: u64) -> Result<Vec<GroupUserDTS>> {
//     let group_users = select_group_user(gid);
//     if group_users.is_empty() {
//         return Ok(Vec::new());
//     }

//     let uids = group_users.iter().map(|d| d.uid).collect::<Vec<u64>>();


// }
