use super::RichTextElementType;
use serde::Serialize;

/// [Rich text preformatted element](https://api.slack.com/reference/block-kit/blocks#rich_text_preformatted)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::{RichTextPreformatted, RichTextElementTypeText};
/// let preformatted = RichTextPreformatted::builder()
///     .element(
///         RichTextElementTypeText::builder()
///             .text("{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}")
///             .build()
///     )
///     .border(0)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_preformatted",
///     "elements": [
///         {
///             "type": "text",
///             "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(preformatted).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextPreformatted {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<RichTextElementType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) border: Option<i64>,
}
