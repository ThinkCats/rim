use rocket::serde::json::Json;
use rocket::{get, post};

use crate::common::resp::{response, wrap_result, WebResponse};

use super::{
    user_model::User,
    user_service::{create_user, query_user},
};

#[get("/get?<uid>")]
pub fn user_get(uid: u64) -> WebResponse<User> {
    let user = query_user(uid);
    response(user, "user not found".into())
}

#[post("/create", data = "<user>")]
pub fn user_create(user: Json<User>) -> WebResponse<u64> {
    let result = create_user(&user);
    wrap_result(result)
}
