use super::HighlightableStyle;
use serde::Serialize;

/// [**channel**](https://api.slack.com/reference/block-kit/blocks#channel-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeChannel, HighlightableStyle};
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
