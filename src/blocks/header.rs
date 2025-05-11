use super::composition_objects::Text;
use serde::Serialize;

/// [Header block](https://docs.slack.dev/reference/block-kit/blocks/header-block)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Header;
/// let header = Header::builder()
///     .text("Budget Performance")
///     .block_id("header_1")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "header",
///     "block_id": "header_1",
///     "text": {
///         "type": "plain_text",
///         "text": "Budget Performance"
///     }
/// });
///
/// let json = serde_json::to_value(header).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) text: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}
