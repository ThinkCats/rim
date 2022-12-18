use rocket::serde::json::Json;
use rocket::{get, post};

use crate::common::resp::{response, wrap_result, WebResponse};

use super::user_model::UserLoginForm;
use super::user_service::{self, query_user_by_token};
use super::{
    user_model::User,
    user_service::{create_user, query_user},
};

#[get("/get?<uid>")]
pub fn user_get(uid: u64) -> WebResponse<User> {
    let result = query_user(uid);
    response(result, 1001, "user not found".into())
}

#[post("/create", data = "<user>")]
pub fn user_create(user: Json<User>) -> WebResponse<u64> {
    let result = create_user(&user);
    wrap_result(result)
}

#[post("/login", data = "<login>")]
pub fn user_login(login: Json<UserLoginForm>) -> WebResponse<String> {
    let result = user_service::login(&login);
    wrap_result(result)
}

#[get("/token?<token>")]
pub fn user_token(token: String) -> WebResponse<User> {
    let result = query_user_by_token(token);
    response(result, 1001, "token not found".into())
}
