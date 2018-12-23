use crate::methods::method::*;
use crate::types::{InlineQueryResult, Integer};

/// Use this method to send answers to an inline query
///
/// No more than 50 results per query are allowed
#[derive(Clone, Debug, Serialize)]
pub struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_personal: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<String>,
}

impl AnswerInlineQuery {
    /// Creates a new AnswerInlineQuery with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * inline_query_id - Unique identifier for the answered query
    /// * results - An array of results for the inline query
    pub fn new<S: Into<String>>(inline_query_id: S, results: Vec<InlineQueryResult>) -> Self {
        AnswerInlineQuery {
            inline_query_id: inline_query_id.into(),
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    /// Maximum amount of time in seconds that the result of the inline query may be cached on the server
    ///
    /// Defaults to 300
    pub fn cache_time(&mut self, cache_time: Integer) -> &mut Self {
        self.cache_time = Some(cache_time);
        self
    }

    /// Cache results on the server side only for the user that sent the query
    ///
    /// By default, results may be returned to any user who sends the same query
    pub fn personal(&mut self, is_personal: bool) -> &mut Self {
        self.is_personal = Some(is_personal);
        self
    }

    /// Offset that a clien should send in the next query with the same text to receive more results
    ///
    /// Pass an empty string if there are no more results or if you don‘t support pagination
    /// Offset length can’t exceed 64 bytes
    pub fn next_offset<S: Into<String>>(&mut self, next_offset: S) -> &mut Self {
        self.next_offset = Some(next_offset.into());
        self
    }

    /// Clients will display a button with specified text that switches the user
    /// to a private chat with the bot and sends the bot a
    /// start message with the parameter switch_pm_parameter
    pub fn switch_pm_text<S: Into<String>>(&mut self, switch_pm_text: S) -> &mut Self {
        self.switch_pm_text = Some(switch_pm_text.into());
        self
    }

    /// Deep-linking parameter for the /start message sent to the bot when user presses the switch button
    ///
    /// 1-64 characters, only A-Z, a-z, 0-9, _ and - are allowed
    ///
    /// Example: An inline bot that sends YouTube videos can ask the user to connect the bot to
    /// their YouTube account to adapt search results accordingly
    /// To do this, it displays a ‘Connect your YouTube account’
    /// button above the results, or even before showing any
    /// The user presses the button, switches to a private chat with the bot and, in doing so,
    /// passes a start parameter that instructs the bot to return an oauth link
    /// Once done, the bot can offer a switch_inline button so that the user can easily
    /// return to the chat where they wanted to use the bot's inline capabilities
    pub fn switch_pm_parameter<S: Into<String>>(&mut self, switch_pm_parameter: S) -> &mut Self {
        self.switch_pm_parameter = Some(switch_pm_parameter.into());
        self
    }
}

impl Method for AnswerInlineQuery {
    type Response = bool;

    fn get_request(&self) -> Result<Request, RequestError> {
        Ok(Request {
            method: RequestMethod::Post,
            url: RequestUrl::new("answerInlineQuery"),
            body: RequestBody::json(&self)?,
        })
    }
}
