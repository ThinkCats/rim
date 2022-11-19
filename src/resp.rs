use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    pub ok: bool,
    pub msg: Option<String>,
    pub data: Option<T>,
}

pub type WebResponse<T> = Json<Response<T>>;

pub fn ok<T>(t: T) -> Response<T> {
    Response {
        ok: true,
        msg: None,
        data: Some(t),
    }
}

pub fn fail<T>(msg: String) -> Response<T> {
    Response {
        ok: false,
        msg: Some(msg),
        data: None,
    }
}

pub fn response<T>(data: Option<T>, fail_msg: String) -> WebResponse<T> {
    match data {
        Some(d) => {
            return json_ok(d);
        }
        None => {
            return json_fail(fail_msg);
        }
    }
}

pub fn json_ok<T>(t: T) -> WebResponse<T> {
    Json(ok(t))
}

pub fn json_fail<T>(msg: String) -> WebResponse<T> {
    Json(fail(msg))
}
