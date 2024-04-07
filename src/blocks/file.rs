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
    kind: &'static str,

    external_id: String,

    source: FileSource,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl File {
    /// Construct a [`FileBuilder`].
    pub fn builder() -> FileBuilder {
        FileBuilder::default()
    }
}

/// Builder for [`FileBuilder`] object.
#[derive(Debug, Default)]
pub struct FileBuilder {
    external_id: Option<String>,
    source: Option<FileSource>,
    block_id: Option<String>,
}

impl FileBuilder {
    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::{File, FileSource};
    /// let file = File::builder()
    ///     .set_block_id(Some("file_block".into()))
    ///     .external_id("ABCD1")
    ///     .source(FileSource::Remote)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file",
    ///     "block_id": "file_block",
    ///     "external_id": "ABCD1",
    ///     "source": "remote"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::{File, FileSource};
    /// let file = File::builder()
    ///     .block_id("file_block")
    ///     .external_id("ABCD1")
    ///     .source(FileSource::Remote)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file",
    ///     "block_id": "file_block",
    ///     "external_id": "ABCD1",
    ///     "source": "remote"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set external_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::{File, FileSource};
    /// let file = File::builder()
    ///     .set_external_id(Some("ABCD1".into()))
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
    pub fn set_external_id(self, external_id: Option<String>) -> Self {
        Self {
            external_id,
            ..self
        }
    }

    /// Set external_id field.
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
    pub fn external_id(self, external_id: impl Into<String>) -> Self {
        self.set_external_id(Some(external_id.into()))
    }

    /// Set source field.
    ///
    /// ```
    /// # use slack_messaging::blocks::{File, FileSource};
    /// let file = File::builder()
    ///     .external_id("ABCD1")
    ///     .set_source(Some(FileSource::Remote))
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
    pub fn set_source(self, source: Option<FileSource>) -> Self {
        Self { source, ..self }
    }

    /// Set source field.
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
    pub fn source(self, source: FileSource) -> Self {
        self.set_source(Some(source))
    }

    /// Build a [`File`] object. This method will panic if external_id and source are not set.
    pub fn build(self) -> File {
        File {
            kind: "file",
            external_id: self
                .external_id
                .expect("external_id must be set to FileBuilder"),
            source: self.source.expect("source must be set to FileBuilder"),
            block_id: self.block_id,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileSource {
    Remote,
}
