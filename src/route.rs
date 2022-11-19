use rocket::{get, routes};

use crate::{
    resp::{response, WebResponse},
    store::query_group,
    user::{query_user, User},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, hello])
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
