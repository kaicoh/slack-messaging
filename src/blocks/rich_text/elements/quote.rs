use super::RichTextElementType;
use serde::Serialize;

/// [Rich text quote element](https://api.slack.com/reference/block-kit/blocks#rich_text_quote)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::{RichTextQuote, RichTextElementTypeText};
/// let quote = RichTextQuote::builder()
///     .element(
///         RichTextElementTypeText::builder()
///             .text("What we need is good examples in our documentation.")
///             .build()
///     )
///     .border(0)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_quote",
///     "elements": [
///         {
///             "type": "text",
///             "text": "What we need is good examples in our documentation."
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(quote).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextQuote {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<RichTextElementType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) border: Option<i64>,
}
