use super::{blocks::Block, Attachment};
use serde::Serialize;

/// [Message payload](https://api.slack.com/reference/messaging/payload)
/// representation.
///
/// # Example
///
/// See also [Header](crate::blocks::Header), [Section](crate::blocks::Section)
/// and [any other blocks](crate::blocks) to know how to build these blocks.
///
/// ```
/// use slack_messaging::Message;
/// use slack_messaging::blocks::{Header, Section};
/// use serde_json::json;
///
/// let message = Message::new()
///     .set_text("New Paid Time Off request from Fred Enriquez")
///     .push_block(Header::new().text("New request"))
///     .push_block(
///         Section::new()
///             .push_field_mrkdwn("*Type:*\nPaid Time Off")
///             .push_field_mrkdwn("*Created by:*\n<example.com|Fred Enriquez>")
///     )
///     .push_block(
///         Section::new()
///             .push_field_mrkdwn("*When:*\nAug 10 - Aug 13")
///     )
///     .push_block(
///         Section::new()
///             .set_text_mrkdwn("<https://example.com|View request>")
///     );
///
/// let expected = json!({
///     "text": "New Paid Time Off request from Fred Enriquez",
///     "blocks": [
///         {
///             "type": "header",
///             "text": {
///                 "type": "plain_text",
///                 "text": "New request",
///                 "emoji": true
///             }
///         },
///         {
///             "type": "section",
///             "fields": [
///                 {
///                     "type": "mrkdwn",
///                     "text": "*Type:*\nPaid Time Off"
///                 },
///                 {
///                     "type": "mrkdwn",
///                     "text": "*Created by:*\n<example.com|Fred Enriquez>"
///                 }
///             ]
///         },
///         {
///             "type": "section",
///             "fields": [
///                 {
///                     "type": "mrkdwn",
///                     "text": "*When:*\nAug 10 - Aug 13"
///                 }
///             ]
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
/// let message_json = serde_json::to_value(message).unwrap();
///
/// assert_eq!(message_json, expected);
/// ```
#[derive(Debug, Default, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mrkdwn: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    channel: Option<String>,
}

impl Message {
    /// Constructs a Message.
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use serde_json::json;
    ///
    /// let message = Message::new();
    /// let expected = json!({});
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets text field.
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .set_text("New Paid Time Off request from Fred Enriquez");
    ///
    /// let expected = json!({
    ///     "text": "New Paid Time Off request from Fred Enriquez",
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }

    /// Sets blocks field directly. The argument is a vector composed from any objects
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use slack_messaging::blocks::{Header, Section};
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .set_blocks(
    ///         vec![
    ///             Header::new().text("New request").into(),
    ///             Section::new().set_text_mrkdwn("<https://example.com|View request>").into()
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "blocks": [
    ///         {
    ///             "type": "header",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "New request",
    ///                 "emoji": true
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
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_blocks(self, blocks: Vec<Block>) -> Self {
        Self { blocks, ..self }
    }

    /// Adds an object to blocks field. The argument is an any object
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use slack_messaging::blocks::{Header, Section};
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .push_block(Header::new().text("New request"))
    ///     .push_block(Section::new().set_text_mrkdwn("<https://example.com|View request>"));
    ///
    /// let expected = json!({
    ///     "blocks": [
    ///         {
    ///             "type": "header",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "New request",
    ///                 "emoji": true
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
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn push_block<T: Into<Block>>(self, block: T) -> Self {
        let Self { mut blocks, .. } = self;
        blocks.push(block.into());
        Self { blocks, ..self }
    }

    /// Sets attachments field directly. See also [Attachment](crate::attachment::Attachment)
    /// to know how to build an Attachment.
    ///
    /// ```
    /// use slack_messaging::{Attachment, Message};
    /// use slack_messaging::blocks::{Context, Section};
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .set_attachments(
    ///         vec![
    ///             Attachment::new()
    ///                 .set_color("#36a64f")
    ///                 .push_block(Context::new().push_element(Text::mrkdwn("*title*")))
    ///                 .push_block(Section::new().set_text_mrkdwn("content"))
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "attachments": [
    ///         {
    ///             "color": "#36a64f",
    ///             "blocks": [
    ///                 {
    ///                     "type": "context",
    ///                     "elements": [
    ///                         {
    ///                             "type": "mrkdwn",
    ///                             "text": "*title*"
    ///                         }
    ///                     ]
    ///                 },
    ///                 {
    ///                     "type": "section",
    ///                     "text": {
    ///                         "type": "mrkdwn",
    ///                         "text": "content"
    ///                     }
    ///                 }
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_attachments(self, attachments: Vec<Attachment>) -> Self {
        Self {
            attachments,
            ..self
        }
    }

    /// Adds an attachment to attachments field.
    /// See also [Attachment](crate::attachment::Attachment) to know
    /// how to build an Attachment.
    ///
    /// ```
    /// use slack_messaging::{Attachment, Message};
    /// use slack_messaging::blocks::{Context, Section};
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .push_attachment(
    ///         Attachment::new()
    ///             .set_color("#36a64f")
    ///             .push_block(Context::new().push_element(Text::mrkdwn("*title*")))
    ///             .push_block(Section::new().set_text_mrkdwn("content"))
    ///
    ///     );
    ///
    /// let expected = json!({
    ///     "attachments": [
    ///         {
    ///             "color": "#36a64f",
    ///             "blocks": [
    ///                 {
    ///                     "type": "context",
    ///                     "elements": [
    ///                         {
    ///                             "type": "mrkdwn",
    ///                             "text": "*title*"
    ///                         }
    ///                     ]
    ///                 },
    ///                 {
    ///                     "type": "section",
    ///                     "text": {
    ///                         "type": "mrkdwn",
    ///                         "text": "content"
    ///                     }
    ///                 }
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn push_attachment(self, attachment: Attachment) -> Self {
        let Self {
            mut attachments, ..
        } = self;
        attachments.push(attachment);
        Self {
            attachments,
            ..self
        }
    }

    /// Alias of [push_attachment](crate::message::Message::push_attachment) method.
    pub fn attach(self, attachment: Attachment) -> Self {
        self.push_attachment(attachment)
    }

    /// Sets thread_ts field.
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use serde_json::json;
    ///
    /// let message = Message::new()
    ///     .set_thread_ts("some ts value");
    ///
    /// let expected = json!({
    ///     "thread_ts": "some ts value",
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_thread_ts<T: Into<String>>(self, thread_ts: T) -> Self {
        Self {
            thread_ts: Some(thread_ts.into()),
            ..self
        }
    }

    /// Sets mrkdwn field.
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use serde_json::json;
    ///
    /// let message = Message::new().set_mrkdwn(true);
    ///
    /// let expected = json!({
    ///     "mrkdwn": true,
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_mrkdwn(self, mrkdwn: bool) -> Self {
        Self {
            mrkdwn: Some(mrkdwn),
            ..self
        }
    }

    /// Sets channel field.
    ///
    /// ```
    /// use slack_messaging::Message;
    /// use serde_json::json;
    ///
    /// let message = Message::new().set_channel("CHANNEL ID");
    ///
    /// let expected = json!({
    ///     "channel": "CHANNEL ID",
    /// });
    ///
    /// let message_json = serde_json::to_value(message).unwrap();
    ///
    /// assert_eq!(message_json, expected);
    /// ```
    pub fn set_channel<T: Into<String>>(self, channel: T) -> Self {
        Self {
            channel: Some(channel.into()),
            ..self
        }
    }
}
