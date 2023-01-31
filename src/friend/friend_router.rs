use rocket::{get, post, serde::json::Json};

use crate::{
    common::{
        resp::{wrap_result, WebResponse},
        store::get_thread_local,
    },
    user::user_model::User,
};

use super::{
    friend_model::FriendAddForm,
    friend_service::{add_friend, list_friend},
};

#[post("/add", data = "<add_form>")]
pub fn friend_add(add_form: Json<FriendAddForm>) -> WebResponse<bool> {
    let store = get_thread_local();
    let mut new_add_form = add_form.clone();
    new_add_form.uid = Some(store.uid);

    let result = add_friend(&new_add_form);
    wrap_result(result)
}

#[get("/list?<status>")]
pub fn friend_list(status: u8) -> WebResponse<Vec<User>> {
    let store = get_thread_local();
    let uid = store.uid;
    let result = list_friend(uid, status);
    wrap_result(result)
}
