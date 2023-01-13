use rocket::{post, serde::json::Json};

use crate::common::{
    resp::{wrap_result, WebResponse},
    store::get_thread_local,
};

use super::{friend_model::FriendAddForm, friend_service::add_friend};

#[post("/add", data = "<add_form>")]
pub fn friend_add(add_form: Json<FriendAddForm>) -> WebResponse<bool> {
    let store = get_thread_local();
    let result = add_friend(store.uid, add_form.uid);
    wrap_result(result)
}
