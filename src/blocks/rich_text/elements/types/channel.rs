use super::HighlightableStyle;
use serde::Serialize;

/// [**channel**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#channel-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeChannel, HighlightableStyle};
/// let channel = RichTextElementTypeChannel::builder()
///     .channel_id("channel-0")
///     .style(
///         HighlightableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "channel",
///     "channel_id": "channel-0",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(channel).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeChannel {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) channel_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<HighlightableStyle>,
}
