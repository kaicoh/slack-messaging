use serde::Serialize;

/// [**color**](https://api.slack.com/reference/block-kit/blocks#color-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::types::RichTextElementTypeColor;
/// let color = RichTextElementTypeColor::builder()
///     .value("#F405B3")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "color",
///     "value": "#F405B3",
/// });
///
/// let json = serde_json::to_value(color).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeColor {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) value: String,
}
