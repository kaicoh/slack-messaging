use super::Markdown;

impl Markdown {
    /// Construct an [`MarkdownBuilder`].
    pub fn builder() -> MarkdownBuilder {
        MarkdownBuilder::default()
    }
}

/// Builder for [`Markdown`] object.
#[derive(Debug, Default)]
pub struct MarkdownBuilder {
    text: Option<String>,
    block_id: Option<String>,
}

impl MarkdownBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Markdown;
    /// let markdown = Markdown::builder()
    ///     .set_text(Some("*this text is italicized*".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "markdown",
    ///     "text": "*this text is italicized*"
    /// });
    ///
    /// let json = serde_json::to_value(markdown).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Markdown;
    /// let markdown = Markdown::builder()
    ///     .text("*this text is italicized*")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "markdown",
    ///     "text": "*this text is italicized*"
    /// });
    ///
    /// let json = serde_json::to_value(markdown).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Markdown;
    /// let markdown = Markdown::builder()
    ///     .set_block_id(Some("markdown-0".into()))
    ///     .set_text(Some("***all of this is important***".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "markdown",
    ///     "block_id": "markdown-0",
    ///     "text": "***all of this is important***"
    /// });
    ///
    /// let json = serde_json::to_value(markdown).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Markdown;
    /// let markdown = Markdown::builder()
    ///     .block_id("markdown-0")
    ///     .text("***all of this is important***")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "markdown",
    ///     "block_id": "markdown-0",
    ///     "text": "***all of this is important***"
    /// });
    ///
    /// let json = serde_json::to_value(markdown).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build an [`Markdown`] object. This method will panic if `text` is not set.
    pub fn build(self) -> Markdown {
        Markdown {
            kind: "markdown",
            text: self.text.expect("text must be set to MarkdownBuilder"),
            block_id: self.block_id,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }
}
