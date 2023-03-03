use super::blocks::Block;
use serde::Serialize;

/// [Secondary message attachment](https://api.slack.com/reference/messaging/attachments)
/// representation.
///
/// # Example
///
/// See also [Context](crate::blocks::Context), [Section](crate::blocks::Section)
/// and [any other blocks](crate::blocks) to know how to build these blocks.
///
/// ```ignore
/// use slack_messaging::Attachment;
/// use slack_messaging::blocks::{Context, Section};
/// use slack_messaging::blocks::elements::{Image, Text};
/// use serde_json::json;
///
/// let author = Context::new()
///     .push_element(
///         Image::new()
///             .set_image_url("https://placeimg.com/16/16/people")
///             .set_alt_text("author")
///     )
///     .push_element(
///         Text::mrkdwn("*<http://flickr.com/bobby/|author>*")
///     );
///
/// let content = Section::new()
///     .set_text_mrkdwn("Optional `text` that appears within the attachment")
///     .push_field_mrkdwn("*A 1st field's title*\nA 1st field's value")
///     .push_field_mrkdwn("*A 2nd field's title*\nA 2nd field's value")
///     .set_accessory(
///         Image::new()
///             .set_image_url("http://placekitten.com/g/200/200")
///             .set_alt_text("cute kitten")
///     );
///
/// let footer = Context::new()
///     .push_element(
///         Image::new()
///             .set_image_url("https://platform.slack-edge.com/img/default_application_icon.png")
///             .set_alt_text("footer icon")
///     )
///     .push_element(
///         Text::mrkdwn("footer | <!date^1392734382^{date} at {time}|February 18th, 2014 at 6:39 AM PST>")
///     );
///
/// let attachment = Attachment::new()
///     .set_color("#36a64f")
///     .push_block(author)
///     .push_block(content)
///     .push_block(footer);
///
/// let expected = json!({
///     "color": "#36a64f",
///     "blocks": [
///         {
///             "type": "context",
///             "elements": [
///                 {
///                     "type": "image",
///                     "image_url": "https://placeimg.com/16/16/people",
///                     "alt_text": "author"
///                 },
///                 {
///                     "type": "mrkdwn",
///                     "text": "*<http://flickr.com/bobby/|author>*"
///                 }
///             ]
///         },
///         {
///             "type": "section",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "Optional `text` that appears within the attachment"
///             },
///             "fields": [
///                 {
///                     "type": "mrkdwn",
///                     "text": "*A 1st field's title*\nA 1st field's value"
///                 },
///                 {
///                     "type": "mrkdwn",
///                     "text": "*A 2nd field's title*\nA 2nd field's value"
///                 }
///             ],
///             "accessory": {
///                 "type": "image",
///                 "image_url": "http://placekitten.com/g/200/200",
///                 "alt_text": "cute kitten"
///             }
///         },
///         {
///             "type": "context",
///             "elements": [
///                 {
///                     "type": "image",
///                     "image_url": "https://platform.slack-edge.com/img/default_application_icon.png",
///                     "alt_text": "footer icon"
///                 },
///                 {
///                     "type": "mrkdwn",
///                     "text": "footer | <!date^1392734382^{date} at {time}|February 18th, 2014 at 6:39 AM PST>"
///                 }
///             ]
///         }
///     ]
/// });
///
/// let attachment_json = serde_json::to_value(attachment).unwrap();
///
/// assert_eq!(attachment_json, expected);
/// ```
#[derive(Debug, Default, Clone, Serialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl Attachment {
    /// Constructs an Attachment.
    ///
    /// ```ignore
    /// use slack_messaging::Attachment;
    /// use serde_json::json;
    ///
    /// let attachment = Attachment::new();
    /// let expected = json!({});
    /// let attachment_json = serde_json::to_value(attachment).unwrap();
    ///
    /// assert_eq!(attachment_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets color field.
    ///
    /// ```ignore
    /// use slack_messaging::Attachment;
    /// use serde_json::json;
    ///
    /// let attachment = Attachment::new().set_color("#FFFFFF");
    /// let expected = json!({
    ///     "color": "#FFFFFF"
    /// });
    /// let attachment_json = serde_json::to_value(attachment).unwrap();
    ///
    /// assert_eq!(attachment_json, expected);
    /// ```
    pub fn set_color<T: Into<String>>(self, color: T) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    /// Sets blocks field directly. The argument is a vector composed from any objects
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```ignore
    /// use slack_messaging::Attachment;
    /// use slack_messaging::blocks::{Context, Section};
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let attachment = Attachment::new()
    ///     .set_blocks(
    ///         vec![
    ///             Context::new().push_element(Text::mrkdwn("*title*")).into(),
    ///             Section::new().set_text_mrkdwn("content").into()
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "blocks": [
    ///         {
    ///             "type": "context",
    ///             "elements": [
    ///                 {
    ///                     "type": "mrkdwn",
    ///                     "text": "*title*"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "section",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "content"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let attachment_json = serde_json::to_value(attachment).unwrap();
    ///
    /// assert_eq!(attachment_json, expected);
    /// ```
    pub fn set_blocks(self, blocks: Vec<Block>) -> Self {
        Self { blocks, ..self }
    }

    /// Adds an object to blocks field. The argument is an any object
    /// that can transform into the enum [Block](crate::blocks::Block).
    ///
    /// ```ignore
    /// use slack_messaging::Attachment;
    /// use slack_messaging::blocks::{Context, Section};
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let attachment = Attachment::new()
    ///     .push_block(Context::new().push_element(Text::mrkdwn("*title*")))
    ///     .push_block(Section::new().set_text_mrkdwn("content"));
    ///
    /// let expected = json!({
    ///     "blocks": [
    ///         {
    ///             "type": "context",
    ///             "elements": [
    ///                 {
    ///                     "type": "mrkdwn",
    ///                     "text": "*title*"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "section",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "content"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let attachment_json = serde_json::to_value(attachment).unwrap();
    ///
    /// assert_eq!(attachment_json, expected);
    /// ```
    pub fn push_block<T: Into<Block>>(self, block: T) -> Self {
        let Self { mut blocks, .. } = self;
        blocks.push(block.into());
        Self { blocks, ..self }
    }
}
