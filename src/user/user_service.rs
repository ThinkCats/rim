use anyhow::{bail, Ok, Result};
use chrono::{Duration, Local, NaiveDateTime};
use uuid::Uuid;

use crate::{
    common::time::{format_time, parse_time_str},
    user::user_dao::update_token,
};

use super::{
    user_dao::{
        has_account, insert_token, insert_user, select_user_by_account, select_user_by_uids,
        select_user_token_by_token, select_user_token_by_uid,
    },
    user_model::{User, UserLoginForm, UserToken},
};

pub fn query_user(uid: u64) -> Option<User> {
    let user = select_user_by_uids(vec![uid]);
    if user.is_err() {
        return None;
    }
    let u = user.unwrap();
    Some(u[0].clone())
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

pub fn valid_token(token: String) -> (bool, Option<u64>) {
    if token.is_empty() {
        return (false, None);
    }
    let user_token = select_user_token_by_token(token);
    match user_token {
        Some(d) => {
            let expire_time = d.expire_time;
            let valid_expire_time = in_expire_time(expire_time);
            return (valid_expire_time, Some(d.u_id));
        }
        None => (false, None),
    }
}

fn create_token(uid: u64) -> String {
    //TODO check user token existed
    let user_token = select_user_token_by_uid(uid);
    match user_token {
        Some(u) => {
            let expire_time = u.expire_time;
            let valid_expire_time = in_expire_time(expire_time);
            if valid_expire_time {
                return u.token;
            }
            //update token
            let token = Uuid::new_v4().to_string();
            let expire_time = calc_token_expire_time();
            let user_token = UserToken {
                id: u.id,
                u_id: u.u_id,
                token: token.clone(),
                expire_time,
            };
            let _ = update_token(&user_token);
            token
        }
        None => {
            let token = Uuid::new_v4().to_string();
            let expire_time = calc_token_expire_time();
            let user_token = UserToken::init(uid, token.clone(), expire_time);
            let _ = insert_token(&user_token);
            token
        }
    }
}

fn calc_token_expire_time() -> String {
    let expire_time = Local::now()
        .checked_add_signed(Duration::days(1))
        .unwrap()
        .naive_local();
    format_time(expire_time)
}

fn in_expire_time(expire_time: String) -> bool {
    let expire = parse_time_str(expire_time);
    println!("{}", expire);
    let now_local = Local::now();
    let now = NaiveDateTime::new(now_local.date_naive(), now_local.time());
    return now.timestamp_millis() < expire.timestamp_millis();
}
