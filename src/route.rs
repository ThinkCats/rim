use rocket::{get, routes, catchers, catch};

use crate::{
    resp::{response, WebResponse, json_fail},
    store::query_group,
    user::{query_user, User},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello world"
}

#[get("/user/get?<uid>")]
fn hello(uid: u64) -> WebResponse<User> {
    query_group(uid);
    let user = query_user(uid);
    response(user, "user not found".into())
}

#[catch(404)]
fn not_found() -> WebResponse<String> {
    json_fail("invalid request".into())
}
