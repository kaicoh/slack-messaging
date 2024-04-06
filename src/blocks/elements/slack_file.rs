use serde::Serialize;

/// [Slack file object](https://api.slack.com/reference/block-kit/composition-objects#slack_file)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SlackFile;
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
    id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl SlackFile {
    /// Construct a [`SlackFileBuilder`].
    pub fn builder() -> SlackFileBuilder {
        SlackFileBuilder::default()
    }
}

/// Builder for [`SlackFile`] object.
#[derive(Debug, Default)]
pub struct SlackFileBuilder {
    id: Option<String>,
    url: Option<String>,
}

impl SlackFileBuilder {
    /// Set id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SlackFile;
    /// let file = SlackFile::builder()
    ///     .set_id(Some("F0123456".into()))
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
    pub fn set_id(self, id: Option<String>) -> Self {
        Self { id, ..self }
    }

    /// Set id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SlackFile;
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
    pub fn id(self, id: impl Into<String>) -> Self {
        self.set_id(Some(id.into()))
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SlackFile;
    /// let file = SlackFile::builder()
    ///     .set_url(Some("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_url(self, url: Option<String>) -> Self {
        Self { url, ..self }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SlackFile;
    /// let file = SlackFile::builder()
    ///     .url("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url.into()))
    }

    /// Build a [`SlackFile`] object. This method will panic if neither `id` nor `url` are set.
    pub fn build(self) -> SlackFile {
        if self.id.is_none() && self.url.is_none() {
            panic!("Either id or url must be set to SlackFileBuilder");
        }

        SlackFile {
            id: self.id,
            url: self.url,
        }
    }
}
