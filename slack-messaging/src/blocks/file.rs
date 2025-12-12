use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [File block](https://docs.slack.dev/reference/block-kit/blocks/file-block)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::{File, FileSource};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let file = File::builder()
///     .external_id("ABCD1")
///     .source(FileSource::Remote)
///     .build()?;
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
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "file")]
pub struct File {
    #[builder(validate("required"))]
    pub(crate) external_id: Option<String>,

    #[builder(validate("required"))]
    pub(crate) source: Option<FileSource>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

/// Values that can be set to the source field of [File].
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileSource {
    Remote,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = File {
            external_id: Some("ABCD1".into()),
            source: Some(FileSource::Remote),
            block_id: Some("file_0".into()),
        };

        let val = File::builder()
            .set_external_id(Some("ABCD1"))
            .set_source(Some(FileSource::Remote))
            .set_block_id(Some("file_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = File::builder()
            .external_id("ABCD1")
            .source(FileSource::Remote)
            .block_id("file_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_external_id_field() {
        let err = File::builder()
            .source(FileSource::Remote)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "File");

        let errors = err.field("external_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_source_field() {
        let err = File::builder().external_id("ABCD1").build().unwrap_err();
        assert_eq!(err.object(), "File");

        let errors = err.field("source");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = File::builder()
            .external_id("ABCD1")
            .source(FileSource::Remote)
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "File");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }
}
