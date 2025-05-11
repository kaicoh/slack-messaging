use serde::Serialize;

/// [Markdown block](https://docs.slack.dev/reference/block-kit/blocks/markdown-block)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Markdown;
/// let markdown = Markdown::builder()
///     .block_id("markdown-0")
///     .text("**Lots of information here!!**")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "markdown",
///     "block_id": "markdown-0",
///     "text": "**Lots of information here!!**"
/// });
///
/// let json = serde_json::to_value(markdown).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Markdown {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}
