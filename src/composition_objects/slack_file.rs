use serde::Serialize;

/// [Slack file object](https://api.slack.com/reference/block-kit/composition-objects#slack_file)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::SlackFile;
/// let file = SlackFile::builder()
///     .id("F0123456")
///     .build();
///
/// let expected = serde_json::json!({
///     "id": "F0123456"
/// });
///
/// let json = serde_json::to_value(file).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SlackFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) url: Option<String>,
}
