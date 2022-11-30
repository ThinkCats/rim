use rocket::{get, post, serde::json::Json};

use crate::{
    common::{
        resp::{wrap_result, WebResponse},
        store::THREAD_LOCAL,
    },
    group::group_model::{Group, GroupCreateForm},
};

use super::{
    group_model::{GroupUpdateForm, GroupUserChangeForm, GroupUserDTS},
    group_service::{change_group_user, create_group, query_group, query_group_user, update_group},
};

#[post("/create", data = "<create_form>")]
pub fn group_create(create_form: Json<GroupCreateForm>) -> WebResponse<u64> {
    let result = create_group(&create_form);
    wrap_result(result)
}

#[post("/update", data = "<update_form>")]
pub fn group_update(update_form: Json<GroupUpdateForm>) -> WebResponse<bool> {
    let result = update_group(&update_form);
    wrap_result(result)
}

#[get("/get?<uid>")]
pub fn group_get(uid: u64) -> WebResponse<Vec<Group>> {
    let _ = THREAD_LOCAL.with(|r| {
        let d = r.borrow();
        println!("test thread local info:{:?}", d);
        return 123;
    });
    let result = query_group(uid);
    wrap_result(result)
}

#[get("/user/get?<gid>")]
pub fn group_user_get(gid: u64) -> WebResponse<Vec<GroupUserDTS>> {
    let result = query_group_user(gid);
    wrap_result(result)
}

#[post("/user/change", data = "<change_form>")]
pub fn group_user_change(change_form: Json<GroupUserChangeForm>) -> WebResponse<bool> {
    let result = change_group_user(&change_form);
    wrap_result(result)
}
