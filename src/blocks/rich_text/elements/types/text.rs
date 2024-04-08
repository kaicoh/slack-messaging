use super::CodableStyle;
use serde::Serialize;

/// [**text**](https://api.slack.com/reference/block-kit/blocks#text-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeText, CodableStyle};
/// let text = RichTextElementTypeText::builder()
///     .text("hello")
///     .style(
///         CodableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "text",
///     "text": "hello",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(text).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeText {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<CodableStyle>,
}
