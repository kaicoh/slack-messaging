use serde::Serialize;

/// [File block](https://api.slack.com/reference/block-kit/blocks#file)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::{File, FileSource};
/// let file = File::builder()
///     .external_id("ABCD1")
///     .source(FileSource::Remote)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "file",
///     "external_id": "ABCD1",
///     "source": "remote"
/// });
///
/// let json = serde_json::to_value(file).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct File {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) external_id: String,

    pub(super) source: FileSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileSource {
    Remote,
}
