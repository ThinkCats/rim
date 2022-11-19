use rocket::{catch, catchers, get, post, routes, serde::json::Json};

use crate::{
    resp::{json_fail, response, WebResponse},
    store::query_group,
    user::{create_user, query_user, User},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, user_get, user_create])
        .register("/", catchers![not_found, server_error])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/user/get?<uid>")]
fn user_get(uid: u64) -> WebResponse<User> {
    query_group(uid);
    let user = query_user(uid);
    response(user, "user not found".into())
}

#[post("/user/create", data = "<user>")]
fn user_create(user: Json<User>) -> WebResponse<u64> {
    let result = create_user(&user);
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
