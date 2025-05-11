use serde::Serialize;

/// [**date**](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#date-element-type)
/// type of [Rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
/// let date = RichTextElementTypeDate::builder()
///     .timestamp(1720710212)
///     .format("{date_num} at {time}")
///     .fallback("timey")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "date",
///     "timestamp": 1720710212,
///     "format": "{date_num} at {time}",
///     "fallback": "timey"
/// });
///
/// let json = serde_json::to_value(date).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeDate {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) timestamp: i64,

    pub(super) format: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) link: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) fallback: Option<String>,
}
