use super::{FileInput, FileType};

impl FileInput {
    /// Construct a [`FileInputBuilder`].
    pub fn builder() -> FileInputBuilder {
        FileInputBuilder::default()
    }
}

/// Builder for [`FileInput`] object.
#[derive(Debug, Default)]
pub struct FileInputBuilder {
    action_id: Option<String>,
    file_types: Vec<FileType>,
    max_files: Option<i64>,
}

impl FileInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .set_action_id(Some("file_input_action_id_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "action_id": "file_input_action_id_1"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .action_id("file_input_action_id_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "action_id": "file_input_action_id_1"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set file_types field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FileInput, FileType};
    /// let file = FileInput::builder()
    ///     .set_file_types(vec![FileType::Jpg, FileType::Png])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "file_types": ["jpg", "png"]
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_file_types(self, file_types: Vec<FileType>) -> Self {
        Self { file_types, ..self }
    }

    /// Add file_type to file_types field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FileInput, FileType};
    /// let file = FileInput::builder()
    ///     .file_type(FileType::Jpg)
    ///     .file_type(FileType::Png)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "file_types": ["jpg", "png"]
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn file_type(self, file_type: FileType) -> Self {
        let Self { mut file_types, .. } = self;
        file_types.push(file_type);
        Self { file_types, ..self }
    }

    /// Set max_files field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .set_max_files(Some(10))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "max_files": 10
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_files(self, max_files: Option<i64>) -> Self {
        Self { max_files, ..self }
    }

    /// Set max_files field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .max_files(10)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "max_files": 10
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_files(self, max_files: impl Into<i64>) -> Self {
        self.set_max_files(Some(max_files.into()))
    }

    /// Build a [`FileInput`] object.
    pub fn build(self) -> FileInput {
        FileInput {
            kind: "file_input",
            action_id: self.action_id,
            file_types: self.file_types,
            max_files: self.max_files,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get file_types value.
    pub fn get_file_types(&self) -> &[FileType] {
        &self.file_types
    }

    /// Get max_files value.
    pub fn get_max_files(&self) -> &Option<i64> {
        &self.max_files
    }
}
