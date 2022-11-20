use anyhow::Result;
use rocket::{catch, catchers, get, post, routes, serde::json::Json};

use crate::{
    resp::{json_fail, response, WebResponse},
    user::{create_user, query_user, User}, group::{GroupCreateForm, create_group},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/user", routes![user_get, user_create])
        .mount("/group", routes![group_create])
        .register("/", catchers![not_found, server_error])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/get?<uid>")]
fn user_get(uid: u64) -> WebResponse<User> {
    let user = query_user(uid);
    response(user, "user not found".into())
}

#[post("/create", data = "<user>")]
fn user_create(user: Json<User>) -> WebResponse<u64> {
    let result = create_user(&user);
    wrap_result(result)
}

#[post("/create", data = "<group>")]
fn group_create(group: Json<GroupCreateForm>) -> WebResponse<u64> {
    let result = create_group(&group);
    wrap_result(result)
}


fn wrap_result<T>(result: Result<T>) -> WebResponse<T> {
    match result {
        Ok(data) => response(Some(data), "fail".into()),
        Err(msg) => response(None, msg.to_string()),
    } 
}


#[catch(404)]
fn not_found() -> WebResponse<String> {
    json_fail("invalid request".into())
}

#[catch(500)]
fn server_error() -> WebResponse<String> {
    json_fail("system error".into())
}
