use crate::blocks::elements::types::FileType;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [File input element](https://docs.slack.dev/reference/block-kit/block-elements/file-input-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{FileInput, types::FileType};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let input = FileInput::builder()
///     .action_id("file_input_action_id_1")
///     .filetype(FileType::Jpg)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "file_input",
///     "action_id": "file_input_action_id_1",
///     "filetypes": ["jpg"]
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let input = FileInput::builder()
///     .action_id("file_input_action_id_1")
///     .filetype(FileType::Jpg)
///     .max_files(99)
///     .build();
///
/// assert!(input.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "file_input")]
pub struct FileInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "filetype")]
    pub(crate) filetypes: Option<Vec<FileType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1", "integer::max_10"))]
    pub(crate) max_files: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = FileInput {
            action_id: Some("file_input_0".into()),
            filetypes: Some(vec![FileType::Jpg, FileType::Png]),
            max_files: Some(5),
        };

        let val = FileInput::builder()
            .set_action_id(Some("file_input_0"))
            .set_filetypes(Some(vec![FileType::Jpg, FileType::Png]))
            .set_max_files(Some(5))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = FileInput::builder()
            .action_id("file_input_0")
            .filetypes(vec![FileType::Jpg, FileType::Png])
            .max_files(5)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = FileInput {
            action_id: None,
            filetypes: Some(vec![FileType::Jpg, FileType::Png]),
            max_files: None,
        };

        let val = FileInput::builder()
            .filetype(FileType::Jpg)
            .filetype(FileType::Png)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = FileInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "FileInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_max_files_greater_than_1() {
        let err = FileInput::builder().max_files(0).build().unwrap_err();
        assert_eq!(err.object(), "FileInput");

        let errors = err.field("max_files");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_max_files_less_than_10() {
        let err = FileInput::builder().max_files(11).build().unwrap_err();
        assert_eq!(err.object(), "FileInput");

        let errors = err.field("max_files");
        assert!(errors.includes(ValidationErrorKind::MaxIntegerValue(10)));
    }
}
