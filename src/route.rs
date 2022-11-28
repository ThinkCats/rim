use anyhow::Result;
use rocket::{catch, catchers, fairing::AdHoc, get, http::uri::Origin, routes, Request};

use crate::{
    common::resp::{json_fail, response, WebResponse},
    group::group_router::{group_create, group_get, group_user_get},
    user::{
        user_router::{user_create, user_get, user_login},
        user_service::valid_token,
    },
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", routes![index, login_need])
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

#[get("/login/need")]
fn login_need() -> WebResponse<String> {
    response(None, 401, "login required".into())
}

fn check_header_token(req: &mut Request) {
    let url_path = req.uri().path().to_string();
    if url_path == "/user/login" {
        return;
    }
    let token = req.headers().get_one("token");
    let valid_token = token.is_some() && valid_token(token.unwrap().into());
    if !valid_token {
        req.set_uri(Origin::parse("/login/need").unwrap());
    }

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
    json_fail(404, "invalid request".into())
}

#[catch(500)]
fn server_error() -> WebResponse<String> {
    json_fail(500, "system error".into())
}
