use rocket::{get, post, serde::json::Json};

use crate::{
    common::resp::{wrap_result, WebResponse},
    group::group_model::{Group, GroupCreateForm},
};

use super::group_service::{create_group, query_group};

#[post("/create", data = "<group>")]
pub fn group_create(group: Json<GroupCreateForm>) -> WebResponse<u64> {
    let result = create_group(&group);
    wrap_result(result)
}

#[get("/get?<uid>")]
pub fn group_get(uid: u64) -> WebResponse<Vec<Group>> {
    let result = query_group(uid);
    wrap_result(result)
}
