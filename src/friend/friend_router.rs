use rocket::{post, serde::json::Json};

use crate::common::resp::WebResponse;

use super::friend_model::FriendAddForm;



#[post("/add",data="<addForm>")]
pub fn friend_add(addForm: Json<FriendAddForm>) -> WebResponse<bool> {
    
    todo!()
}