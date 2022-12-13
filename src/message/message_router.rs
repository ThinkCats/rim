use rocket::{post, serde::json::Json};

use crate::common::resp::{wrap_result, WebResponse};

use super::{
    message_model::{ChatGroupReadForm, ChatListForm, ChatMessage, MessageForm},
    message_service::{query_chat_group_msg_history, query_chat_list_page, update_chat_group_read},
};

#[post("/chat/list", data = "<query_form>")]
pub fn chat_list(query_form: Json<ChatListForm>) -> WebResponse<Vec<ChatMessage>> {
    let result = query_chat_list_page(&query_form);
    wrap_result(result)
}

#[post("/history", data = "<query_form>")]
pub fn history(query_form: Json<MessageForm>) -> WebResponse<Vec<ChatMessage>> {
    let result = query_chat_group_msg_history(&query_form);
    wrap_result(result)
}

#[post("/group/read", data = "<update_form>")]
pub fn chat_group_read(update_form: Json<ChatGroupReadForm>) -> WebResponse<bool> {
    let result = update_chat_group_read(&update_form);
    wrap_result(result)
}
