use anyhow::{Ok, Result};

use crate::{
    common::store::{GROUP_USER_OP_TYPE_ADD, GROUP_USER_OP_TYPE_REMOVE, GROUP_USER_ROLE_COMMON},
    user::user_dao::select_user_by_uids,
};

use super::{
    group_dao::{
        self, delete_group_user, insert_group, insert_group_user, select_group, select_group_user,
    },
    group_model::{
        Group, GroupCreateForm, GroupUpdateForm, GroupUser, GroupUserChangeForm, GroupUserDTS,
    },
};

pub fn create_group(form: &GroupCreateForm) -> Result<u64> {
    insert_group(form)
}

pub fn update_group(form: &GroupUpdateForm) -> Result<bool> {
    group_dao::update_group(form)
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

pub fn change_group_user(change_form: &GroupUserChangeForm) -> Result<bool> {
    let group_users = convert_group_users(change_form);
    let existed_group_user = select_group_user(change_form.gid);
    let op_type = change_form.op_type;
    if op_type == GROUP_USER_OP_TYPE_ADD {
        //add user
        let to_add_group_users = filter_group_user(group_users, existed_group_user, false);
        if !to_add_group_users.is_empty() {
            let _ = insert_group_user(to_add_group_users);
        }
    } else if op_type == GROUP_USER_OP_TYPE_REMOVE {
        //remove user
        let to_remove_group_users = filter_group_user(group_users, existed_group_user, true);
        if !to_remove_group_users.is_empty() {
            let _ = delete_group_user(to_remove_group_users);
        }
    }
    Ok(true)
}

fn convert_group_users(change_form: &GroupUserChangeForm) -> Vec<GroupUser> {
    change_form
        .uids
        .iter()
        .map(|r| GroupUser {
            id: None,
            gid: change_form.gid,
            uid: *r,
            role: GROUP_USER_ROLE_COMMON,
        })
        .collect::<Vec<GroupUser>>()
}

fn filter_group_user(
    current_group_user: Vec<GroupUser>,
    existed_group_user: Vec<GroupUser>,
    in_existed: bool,
) -> Vec<GroupUser> {
    if existed_group_user.is_empty() {
        if in_existed {
            return Vec::new();
        }
        return current_group_user;
    }
    let mut result: Vec<GroupUser> = Vec::new();
    for ele in &current_group_user {
        for ele_e in &existed_group_user {
            if ele.uid == ele_e.uid {
                if in_existed {
                    result.push(ele_e.clone());
                }
            } else {
                if !in_existed {
                    result.push(ele.clone());
                }
            }
        }
    }
    result
}
