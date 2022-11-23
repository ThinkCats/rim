use anyhow::Result;

use super::{
    group_dao::{insert_group, select_group},
    group_model::{Group, GroupCreateForm},
};

pub fn create_group(form: &GroupCreateForm) -> Result<u64> {
    insert_group(form)
}

pub fn query_group(uid: u64) -> Result<Vec<Group>> {
    select_group(uid)
}
