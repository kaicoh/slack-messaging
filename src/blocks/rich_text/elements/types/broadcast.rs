use serde::Serialize;

/// [**broadcast**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#broadcast-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeBroadcast, BroadcastRange};
/// let broadcast = RichTextElementTypeBroadcast::builder()
///     .range(BroadcastRange::Everyone)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "broadcast",
///     "range": "everyone",
/// });
///
/// let json = serde_json::to_value(broadcast).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeBroadcast {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) range: BroadcastRange,
}

/// values to be set to [RichTextElementTypeBroadcast]'s range field
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastRange {
    Here,
    Channel,
    Everyone,
}
