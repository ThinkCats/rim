use anyhow::Result;
use rocket::{catch, catchers, fairing::AdHoc, get, http::uri::Origin, post, routes, Request};

use crate::{
    common::{
        resp::{json_fail, response, WebResponse},
        store::add_local_store,
    },
    friend::friend_router::{friend_add, friend_list, friend_status_modify},
    group::group_router::{
        group_create, group_get, group_update, group_user_change, group_user_get,
    },
    message::message_router::{chat_group_read, chat_list, history},
    user::{
        user_router::{user_create, user_get, user_login, user_search, user_token},
        user_service::valid_token,
    },
};

pub async fn launch_web() -> Result<(), rocket::Error> {
    let figment = rocket::Config::figment().merge(("port", 8000));
    let _rocket = rocket::custom(figment)
        .mount("/", routes![index, login_need, login_need_post])
        .mount(
            "/api/user",
            routes![user_get, user_create, user_login, user_token, user_search],
        )
        .mount(
            "/api/group",
            routes![
                group_create,
                group_update,
                group_get,
                group_user_get,
                group_user_change
            ],
        )
        .mount("/api/message", routes![chat_list, history, chat_group_read])
        .mount(
            "/api/friend",
            routes![friend_add, friend_list, friend_status_modify],
        )
        .attach(token_checker_adhoc())
        .register("/", catchers![not_found, server_error])
        .launch()
        .await?;

    Ok(())
}

fn token_checker_adhoc() -> AdHoc {
    AdHoc::on_request("token_checker", |req, _| {
        Box::pin(async move {
            println!(
                "token checker start on request:{}",
                req.uri().path().to_string()
            );
            check_header_token(req);
        })
    })
}

#[get("/")]
fn index() -> &'static str {
    "hello rim"
}

#[get("/login/need")]
fn login_need() -> WebResponse<String> {
    response(None, 401, "login required".into())
}

#[post("/login/need")]
fn login_need_post() -> WebResponse<String> {
    response(None, 401, "login required".into())
}

fn check_header_token(req: &mut Request) {
    let url_path = req.uri().path().to_string();
    if url_path == "/api/user/login" || url_path == "/api/user/create" {
        return;
    }
    let auth_token = req.headers().get_one("Authorization");
    if auth_token.is_none() {
        req.set_uri(Origin::parse("/login/need").unwrap());
        return;
    }

    let bare_token = auth_token.unwrap().to_string();
    let token = get_bare_token(bare_token);
    if token.is_none() {
        req.set_uri(Origin::parse("/login/need").unwrap());
        return;
    }
    let token_str = token.unwrap();
    let valid_result = valid_token(token_str.clone());
    if !valid_result.0 {
        req.set_uri(Origin::parse("/login/need").unwrap());
        return;
    }
    //Store login info
    add_local_store(token_str.clone(), valid_result.1.unwrap());
}

fn get_bare_token(bare_token: String) -> Option<String> {
    let split: Vec<&str> = bare_token.split(" ").collect();
    if split.is_empty() || split.len() != 2 {
        return None;
    }
    let token = split[1];
    return Some(token.into());
}

#[catch(404)]
fn not_found() -> WebResponse<String> {
    json_fail(404, "invalid request".into())
}

#[catch(500)]
fn server_error() -> WebResponse<String> {
    json_fail(500, "system error".into())
}
