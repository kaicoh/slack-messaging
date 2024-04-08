use serde::Serialize;

/// [**emoji**](https://api.slack.com/reference/block-kit/blocks#emoji-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::types::RichTextElementTypeEmoji;
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
}
