use crate::common::resp::WebResponse;

use super::message_model::ChatMessage;

pub fn chat_list(uid: u64) -> WebResponse<Vec<ChatMessage>> {
    //todo:  data should get from cache (like redis)  if too much msg stored in database
    
    todo!("")
}
