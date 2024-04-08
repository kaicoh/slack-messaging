use super::HighlightableStyle;
use serde::Serialize;

/// [**user**](https://api.slack.com/reference/block-kit/blocks#user-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeUser, HighlightableStyle};
/// let user = RichTextElementTypeUser::builder()
///     .user_id("user-0")
///     .style(
///         HighlightableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "user",
///     "user_id": "user-0",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(user).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeUser {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) user_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<HighlightableStyle>,
}
