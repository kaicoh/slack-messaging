use super::CodableStyle;
use serde::Serialize;

/// [**link**](https://api.slack.com/reference/block-kit/blocks#link-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeLink, CodableStyle};
/// let link = RichTextElementTypeLink::builder()
///     .url("https://google.com")
///     .style(
///         CodableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "link",
///     "url": "https://google.com",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(link).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeLink {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) r#unsafe: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<CodableStyle>,
}
