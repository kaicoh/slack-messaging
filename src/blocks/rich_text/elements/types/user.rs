use super::HighlightableStyle;
use serde::Serialize;

/// [**user**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeUser, HighlightableStyle};
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
