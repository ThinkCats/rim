use anyhow::{bail, Result};

use super::{user_dao::{select_user, has_account, insert_user}, user_model::User};

pub fn query_user(uid: u64) -> Option<User> {
    select_user(uid)
}

pub fn create_user(user: &User) -> Result<u64> {
    if has_account(&user.account) {
        bail!("Account Existed")
    }
    insert_user(user)
}