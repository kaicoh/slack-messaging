use serde::Serialize;

/// [Slack file object](https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object)
/// representation.
///
/// The Builder returns [`SlackFileError`](crate::composition_objects::builders::SlackFileError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::SlackFile;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let file = SlackFile::builder()
///     .id("F0123456")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "id": "F0123456"
/// });
///
/// let json = serde_json::to_value(file).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct SlackFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
}
