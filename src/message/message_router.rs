use rocket::{post, serde::json::Json};

use crate::common::resp::{wrap_result, WebResponse};

use super::{
    message_model::{ChatListForm, ChatMessage},
    message_service::query_chat_list_page,
};

#[post("/chat/list", data = "<query_form>")]
pub fn chat_list(query_form: Json<ChatListForm>) -> WebResponse<Vec<ChatMessage>> {
    let result = query_chat_list_page(&query_form);
    wrap_result(result)
}
