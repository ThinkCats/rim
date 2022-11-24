use anyhow::{bail, Result};

use super::{user_dao::{select_user, has_account, insert_user}, user_model::User};

pub fn query_user(uid: u64) -> Option<User> {
     let r = select_user(vec![uid]);
     if r.is_none() {
         return None;
     }
     let a = r.unwrap()[0].clone();
     Some(a)
}

pub fn create_user(user: &User) -> Result<u64> {
    if has_account(&user.account) {
        bail!("Account Existed")
    }
    insert_user(user)
}