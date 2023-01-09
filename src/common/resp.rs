use anyhow::Result;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub ok: bool,
    pub code: u32,
    pub msg: Option<String>,
    pub data: Option<T>,
}

pub type WebResponse<T> = Json<Response<T>>;

pub fn ok<T>(t: T) -> Response<T> {
    Response {
        ok: true,
        code: 200,
        msg: None,
        data: Some(t),
    }
}

pub fn fail<T>(code: u32, msg: String) -> Response<T> {
    Response {
        ok: false,
        code,
        msg: Some(msg),
        data: None,
    }
}

pub fn wrap_result<T>(result: Result<T>) -> WebResponse<T> {
    match result {
        Ok(data) => response(Some(data), 200, "fail".into()),
        Err(msg) => response(None, 500, msg.to_string()),
    }
}

pub fn wrap_option<T>(result: Option<T>) -> WebResponse<T> {
    response(result, 200, "fail".into())
}

pub fn response<T>(data: Option<T>, code: u32, fail_msg: String) -> WebResponse<T> {
    match data {
        Some(d) => {
            return json_ok(d);
        }
        None => {
            return json_fail(code, fail_msg);
        }
    }
}

pub fn json_ok<T>(t: T) -> WebResponse<T> {
    Json(ok(t))
}

pub fn json_fail<T>(code: u32, msg: String) -> WebResponse<T> {
    Json(fail(code, msg))
}
