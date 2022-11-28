use rocket::{get, post, serde::json::Json};

use crate::{
    common::{resp::{wrap_result, WebResponse}, store::THREAD_LOCAL},
    group::group_model::{Group, GroupCreateForm},
};

use super::{group_service::{create_group, query_group, query_group_user}, group_model::{GroupUserDTS, GroupUserChangeForm}};

#[post("/create", data = "<create_form>")]
pub fn group_create(create_form: Json<GroupCreateForm>) -> WebResponse<u64> {
    let result = create_group(&create_form);
    wrap_result(result)
}

#[get("/get?<uid>")]
pub fn group_get(uid: u64) -> WebResponse<Vec<Group>> {
    let _ = THREAD_LOCAL.with(|r| {
        let d = r.borrow();
        println!("test thread local info:{:?}",d);
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
    //TODO
    wrap_result(Ok(true))
}
