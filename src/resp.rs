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

pub fn json_ok<T>(t: T) -> Json<Response<T>> {
    Json(ok(t))
}

pub fn json_fail<T>(msg: String) -> Json<Response<T>> {
    Json(fail(msg))
}
