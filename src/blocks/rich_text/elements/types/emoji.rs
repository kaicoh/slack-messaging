use serde::Serialize;

/// [**emoji**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#emoji-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeEmoji;
/// let emoji = RichTextElementTypeEmoji::builder()
///     .name("wave")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "emoji",
///     "name": "wave"
/// });
///
/// let json = serde_json::to_value(emoji).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeEmoji {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) unicode: Option<String>,
}
