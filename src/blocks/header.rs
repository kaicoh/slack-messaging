use super::elements::Text;
use crate::plain_text;
use serde::Serialize;

/// [Header block](https://api.slack.com/reference/block-kit/blocks#header)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::Header;
/// use serde_json::json;
///
/// let header = Header::new()
///     .text("Budget Performance")
///     .set_block_id("header_1");
///
/// let expected = json!({
///     "type": "header",
///     "block_id": "header_1",
///     "text": {
///         "type": "plain_text",
///         "text": "Budget Performance",
///         "emoji": true
///     }
/// });
///
/// let header_json = serde_json::to_value(header).unwrap();
///
/// assert_eq!(header_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    #[serde(rename = "type")]
    kind: &'static str,

    text: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            kind: "header",
            text: plain_text!(""),
            block_id: None,
        }
    }
}

impl Header {
    /// Constructs a Header block.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Header;
    /// use serde_json::json;
    ///
    /// let header = Header::new();
    ///
    /// let expected = json!({
    ///     "type": "header",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let header_json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(header_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Header;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let header = Header::new()
    ///     .set_text(Text::plain("Budget Performance"));
    ///
    /// let expected = json!({
    ///     "type": "header",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Budget Performance",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let header_json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(header_json, expected);
    /// ```
    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    /// Sets block_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Header;
    /// use serde_json::json;
    ///
    /// let header = Header::new()
    ///     .set_block_id("header_1");
    ///
    /// let expected = json!({
    ///     "type": "header",
    ///     "block_id": "header_1",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let header_json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(header_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
