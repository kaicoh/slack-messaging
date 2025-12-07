use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, FileInput, FileType};

use std::error::Error;
use std::fmt;

impl FileInput {
    /// Construct a [`FileInputBuilder`].
    pub fn builder() -> FileInputBuilder {
        FileInputBuilder::default()
    }
}

/// Error while building [`FileInput`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FileInputError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of filetypes field
    pub filetypes: Vec<ValidationError>,

    /// errors of max_files field
    pub max_files: Vec<ValidationError>,
}

impl fmt::Display for FileInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FileInputError {{ action_id: {:?}, filetypes: {:?}, max_files: {:?} }}",
            self.action_id, self.filetypes, self.max_files
        )
    }
}

impl Error for FileInputError {}

/// Builder for [`FileInput`] object.
#[derive(Debug)]
pub struct FileInputBuilder {
    action_id: Value<String>,
    filetypes: Value<Vec<FileType>>,
    max_files: Value<i64>,
}

impl Default for FileInputBuilder {
    fn default() -> Self {
        FileInputBuilder {
            action_id: new_action_id(None),
            filetypes: new_filetypes(None),
            max_files: new_max_files(None),
        }
    }
}

impl Builder for FileInputBuilder {
    type Target = FileInput;
    type Error = FileInputError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            filetypes,
            max_files,
        } = self;
        value::merge_3(action_id, filetypes, max_files)
            .map(|(action_id, filetypes, max_files)| FileInput {
                action_id,
                filetypes: filetypes.unwrap_or_default(),
                max_files,
            })
            .map_err(|(action_id, filetypes, max_files)| FileInputError {
                action_id,
                filetypes,
                max_files,
            })
    }
}

impl FileInputBuilder {
    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    pub fn set_action_id(self, action_id: Option<impl Into<String>>) -> Self {
        Self {
            action_id: new_action_id(action_id.map(|v| v.into())),
            ..self
        }
    }

    /// set action_id field value
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id))
    }

    /// get filetypes field value
    pub fn get_filetypes(&self) -> Option<&[FileType]> {
        self.filetypes.inner_ref().map(|v| v.as_ref())
    }

    /// set filetypes field value
    pub fn set_filetypes(self, filetypes: Option<Vec<FileType>>) -> Self {
        Self {
            filetypes: new_filetypes(filetypes),
            ..self
        }
    }

    /// set filetypes field value
    pub fn filetypes(self, filetypes: Vec<FileType>) -> Self {
        self.set_filetypes(Some(filetypes))
    }

    /// add filetype to filetypes field
    pub fn filetype(mut self, filetype: FileType) -> Self {
        let mut list = self.filetypes.take_inner().unwrap_or_default();
        list.push(filetype);
        self.filetypes(list)
    }

    /// get max_files field value
    pub fn get_max_files(&self) -> Option<i64> {
        self.max_files.inner_ref().copied()
    }

    /// set max_files field value
    pub fn set_max_files(self, max_files: Option<i64>) -> Self {
        Self {
            max_files: new_max_files(max_files),
            ..self
        }
    }

    /// set max_files field value
    pub fn max_files(self, max_files: i64) -> Self {
        self.set_max_files(Some(max_files))
    }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_filetypes(options: Option<Vec<FileType>>) -> Value<Vec<FileType>> {
    pipe! { Value::new(options) => validators::do_nothing }
}

fn new_max_files(max_files: Option<i64>) -> Value<i64> {
    pipe! {
        Value::new(max_files) =>
            validators::integer::min_1 |
            validators::integer::max_10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = FileInput {
            action_id: Some("file_input_action_id_1".into()),
            filetypes: vec![FileType::Jpg, FileType::Png],
            max_files: Some(5),
        };

        let val = FileInput::builder()
            .set_action_id(Some("file_input_action_id_1"))
            .set_filetypes(Some(vec![FileType::Jpg, FileType::Png]))
            .set_max_files(Some(5))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = FileInput::builder()
            .action_id("file_input_action_id_1")
            .filetypes(vec![FileType::Jpg, FileType::Png])
            .max_files(5)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_has_additional_setter_for_filetypes_field() {
        let expected = FileInput {
            action_id: Some("file_input_action_id_1".into()),
            filetypes: vec![FileType::Jpg, FileType::Png],
            max_files: Some(5),
        };

        let val = FileInput::builder()
            .action_id("file_input_action_id_1")
            .filetype(FileType::Jpg)
            .filetype(FileType::Png)
            .max_files(5)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_must_be_less_than_255() {
        let err = FileInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();

        let expected = FileInputError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn max_files_field_must_be_greater_than_1() {
        let err = FileInput::builder().max_files(0).build().unwrap_err();

        let expected = FileInputError {
            max_files: vec![ValidationError::MinIntegerValue(1)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn max_files_field_must_be_smaller_than_10() {
        let err = FileInput::builder().max_files(11).build().unwrap_err();

        let expected = FileInputError {
            max_files: vec![ValidationError::MaxIntegerValue(10)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
