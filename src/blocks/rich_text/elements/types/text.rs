use super::CodableStyle;
use serde::Serialize;

/// [**text**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#text-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
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
