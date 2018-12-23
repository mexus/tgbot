use crate::methods::method::*;
use crate::types::{ChatId, ChatMember, Integer};

/// Get information about a member of a chat
#[derive(Clone, Debug, Serialize)]
pub struct GetChatMember {
    chat_id: ChatId,
    user_id: Integer,
}

impl GetChatMember {
    /// Creates a new GetChatMember
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * user_id - Unique identifier of the target user
    pub fn new<C: Into<ChatId>>(chat_id: C, user_id: Integer) -> Self {
        GetChatMember {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl Method for GetChatMember {
    type Response = ChatMember;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("getChatMember"),
            body: RequestBody::json(&self)?,
        })
    }
}
