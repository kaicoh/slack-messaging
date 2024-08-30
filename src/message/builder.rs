use super::{Block, Message};

impl Message {
    /// Construct a [`MessageBuilder`].
    pub fn builder() -> MessageBuilder {
        MessageBuilder::default()
    }
}

/// Builder for [`Message`] object.
#[derive(Debug, Default)]
pub struct MessageBuilder {
    text: Option<String>,
    blocks: Vec<Block>,
    thread_ts: Option<String>,
    mrkdwn: Option<bool>,
    response_type: Option<String>,
    replace_original: Option<bool>,
    delete_original: Option<bool>,
}

impl MessageBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_text(Some("New Paid Time Off request from Fred Enriquez".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": "New Paid Time Off request from Fred Enriquez",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .text("New Paid Time Off request from Fred Enriquez")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": "New Paid Time Off request from Fred Enriquez",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set blocks field. The argument is a vector composed from any objects
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```
    /// # use slack_messaging::{blocks::{Header, Section}, mrkdwn, Message};
    /// let message = Message::builder()
    ///     .set_blocks(
    ///         vec![
    ///             Header::builder()
    ///                 .text("New request")
    ///                 .build()
    ///                 .into(),
    ///             Section::builder()
    ///                 .text(mrkdwn!("<https://example.com|View request>"))
    ///                 .build()
    ///                 .into(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "blocks": [
    ///         {
    ///             "type": "header",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "New request"
    ///             }
    ///         },
    ///         {
    ///             "type": "section",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "<https://example.com|View request>"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_blocks(self, blocks: Vec<Block>) -> Self {
        Self { blocks, ..self }
    }

    /// Add an object to blocks field. The argument is an any object
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```
    /// # use slack_messaging::{blocks::{Header, Section}, mrkdwn, Message};
    /// let message = Message::builder()
    ///     .block(
    ///         Header::builder()
    ///             .text("New request")
    ///             .build()
    ///     )
    ///     .block(
    ///         Section::builder()
    ///             .text(mrkdwn!("<https://example.com|View request>"))
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "blocks": [
    ///         {
    ///             "type": "header",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "New request"
    ///             }
    ///         },
    ///         {
    ///             "type": "section",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "<https://example.com|View request>"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block(self, block: impl Into<Block>) -> Self {
        let Self { mut blocks, .. } = self;
        blocks.push(block.into());
        Self { blocks, ..self }
    }

    /// Set thread_ts field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_thread_ts(Some("some ts value".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "thread_ts": "some ts value",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_thread_ts(self, thread_ts: Option<String>) -> Self {
        Self { thread_ts, ..self }
    }

    /// Set thread_ts field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .thread_ts("some ts value")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "thread_ts": "some ts value",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn thread_ts(self, thread_ts: impl Into<String>) -> Self {
        self.set_thread_ts(Some(thread_ts.into()))
    }

    /// Set mrkdwn field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_mrkdwn(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "mrkdwn": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_mrkdwn(self, mrkdwn: Option<bool>) -> Self {
        Self { mrkdwn, ..self }
    }

    /// Set mrkdwn field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .mrkdwn(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "mrkdwn": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn mrkdwn(self, mrkdwn: bool) -> Self {
        self.set_mrkdwn(Some(mrkdwn))
    }

    /// Set response_type field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_response_type(Some("in_channel".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "response_type": "in_channel",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_response_type(self, response_type: Option<String>) -> Self {
        Self {
            response_type,
            ..self
        }
    }

    /// Set response_type field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .response_type("in_channel")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "response_type": "in_channel",
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn response_type(self, response_type: impl Into<String>) -> Self {
        self.set_response_type(Some(response_type.into()))
    }

    /// Set replace_original field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_replace_original(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "replace_original": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_replace_original(self, replace_original: Option<bool>) -> Self {
        Self {
            replace_original,
            ..self
        }
    }

    /// Set replace_original field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .replace_original(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "replace_original": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn replace_original(self, replace_original: bool) -> Self {
        self.set_replace_original(Some(replace_original))
    }

    /// Set delete_original field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .set_delete_original(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "delete_original": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_delete_original(self, delete_original: Option<bool>) -> Self {
        Self {
            delete_original,
            ..self
        }
    }

    /// Set delete_original field.
    ///
    /// ```
    /// # use slack_messaging::Message;
    /// let message = Message::builder()
    ///     .delete_original(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "delete_original": true,
    /// });
    ///
    /// let json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn delete_original(self, delete_original: bool) -> Self {
        self.set_delete_original(Some(delete_original))
    }

    /// Build a [`Message`] object.
    pub fn build(self) -> Message {
        Message {
            text: self.text,
            blocks: self.blocks,
            thread_ts: self.thread_ts,
            mrkdwn: self.mrkdwn,
            response_type: self.response_type,
            replace_original: self.replace_original,
            delete_original: self.delete_original,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get blocks value.
    pub fn get_blocks(&self) -> &[Block] {
        &self.blocks
    }

    /// Get thread_ts value.
    pub fn get_thread_ts(&self) -> &Option<String> {
        &self.thread_ts
    }

    /// Get mrkdwn value.
    pub fn get_mrkdwn(&self) -> &Option<bool> {
        &self.mrkdwn
    }

    /// Get response_type value.
    pub fn get_response_type(&self) -> &Option<String> {
        &self.response_type
    }

    /// Get replace_original value.
    pub fn get_replace_original(&self) -> &Option<bool> {
        &self.replace_original
    }

    /// Get delete_original value.
    pub fn get_delete_original(&self) -> &Option<bool> {
        &self.delete_original
    }
}
