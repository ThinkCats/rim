use anyhow::Result;
use rocket::{catch, catchers, get, routes};

use crate::{
    common::resp::{json_fail, WebResponse},
    group::group_router::{group_create, group_get, group_user_get},
    user::user_router::{user_create, user_get},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/user", routes![user_get, user_create])
        .mount("/group", routes![group_create, group_get, group_user_get])
        .register("/", catchers![not_found, server_error])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[catch(404)]
fn not_found() -> WebResponse<String> {
    json_fail("invalid request".into())
}

#[catch(500)]
fn server_error() -> WebResponse<String> {
    json_fail("system error".into())
}
