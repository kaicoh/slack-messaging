use super::HighlightableStyle;
use serde::Serialize;

/// [**usergroup**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-group-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeUserGroup, HighlightableStyle};
/// let usergroup = RichTextElementTypeUserGroup::builder()
///     .usergroup_id("usergroup-0")
///     .style(
///         HighlightableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "usergroup",
///     "usergroup_id": "usergroup-0",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(usergroup).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeUserGroup {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) usergroup_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<HighlightableStyle>,
}
