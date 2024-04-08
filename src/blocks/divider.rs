use serde::Serialize;

/// [Divider block](https://api.slack.com/reference/block-kit/blocks#divider)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Divider;
/// let divider = Divider::builder()
///     .block_id("divider_block")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "divider",
///     "block_id": "divider_block"
/// });
///
/// let json = serde_json::to_value(divider).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}
