use rocket::{get, routes, serde::json::Json};

use crate::{store::query_group, user::{query_user, User, Response, ok, fail}};

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
fn hello(uid: u64) -> Json<Response<User>> {
    query_group(uid);
    let user = query_user(uid);
    match user {
        Some(u) => {
            return Json(ok(u));
        }
        None => {
            return Json(fail("User Not Found".into())) ;
        }
    }
}
