use anyhow::Result;
use rocket::{catch, catchers, fairing::AdHoc, get, response::Redirect, routes, uri, Request, http::uri::Origin};

use crate::{
    common::resp::{json_fail, WebResponse},
    group::group_router::{group_create, group_get, group_user_get},
    user::user_router::{user_create, user_get, user_login},
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index])
        .mount("/user", routes![user_get, user_create, user_login])
        .mount("/group", routes![group_create, group_get, group_user_get])
        .attach(AdHoc::on_request("token_checker", |req, _| {
            Box::pin(async move {
                println!(
                    "token checker start on request:{}",
                    req.uri().path().to_string()
                );
                check_header_token(req);
            })
        }))
        .register("/", catchers![not_found, server_error])
        .launch()
        .await?;

    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "hello rim"
}

// #[get("/")]
// fn need_login() -> Result<String> {

// }

fn check_header_token(req: &mut Request) {
    let url_path = req.uri().path().to_string();
    if url_path == "/user/login" {
        return;
    }
    let mut token = req.headers().get_one("token");
    // req.set_uri(Origin::parse("/user/login").unwrap());
    // match token {
    //     Some(t) => {
    //         println!("TODO handle token:{}", t);
    //     }
    //     None => {
    //         println!("redirect to login for none token");
    //         &req.set_uri(Origin::parse("/user/login").unwrap());
    //         // let _ = Redirect::to(uri!("/user/login"));
    //     }
    // }
}

#[catch(404)]
fn not_found() -> WebResponse<String> {
    json_fail(404 ,"invalid request".into())
}

#[catch(500)]
fn server_error() -> WebResponse<String> {
    json_fail(500,"system error".into())
}
