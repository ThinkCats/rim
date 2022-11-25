use anyhow::{bail, Ok, Result};
use uuid::Uuid;

use super::{
    user_dao::{
        has_account, insert_token, insert_user, select_user_by_account, select_user_by_uids,
    },
    user_model::{User, UserLoginForm, UserToken},
};

pub fn query_user(uid: u64) -> Option<User> {
    let user = select_user_by_uids(vec![uid]);
    user.and_then(|r| {
        let u = r[0].clone();
        return Some(u);
    })
}

pub fn create_user(user: &User) -> Result<u64> {
    if has_account(&user.account) {
        bail!("Account Existed")
    }
    insert_user(user)
}

pub fn login(login_form: &UserLoginForm) -> Result<String> {
    let user = select_user_by_account(login_form.account.clone());
    match user {
        Some(u) => {
            let password = u.password.expect("password not found");
            if password != login_form.password {
                bail!("wrong password")
            }
            //create token
            let u_id = u.id.expect("uid not found");
            let token = create_token(u_id);
            Ok(token)
        }
        None => {
            bail!("account not found")
        }
    }
}

fn create_token(uid: u64) -> String {
    //TODO check user token existed
    let token = Uuid::new_v4().to_string();
    //TODO expire time process
    let expire_time = "2022-11-28 00:00:03".into();
    let user_token = UserToken::init(uid, token.clone(), expire_time);
    let _ = insert_token(&user_token);
    token
}
