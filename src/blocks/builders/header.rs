use super::{Header, composition_objects::Text};

impl Header {
    /// Construct a [`HeaderBuilder`].
    pub fn builder() -> HeaderBuilder {
        HeaderBuilder::default()
    }
}

/// Builder for [`Header`] object.
#[derive(Debug, Default)]
pub struct HeaderBuilder {
    text: Option<Text>,
    block_id: Option<String>,
}

impl HeaderBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Header;
    /// # use slack_messaging::composition_objects::Text;
    /// let header = Header::builder()
    ///     .set_text(
    ///         Some(Text::builder()
    ///             .plain_text("Budget Performance")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "header",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Budget Performance"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<Text>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Header;
    /// let header = Header::builder()
    ///     .text("Budget Performance")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "header",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Budget Performance"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(text).build();
        self.set_text(Some(text))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Header;
    /// let header = Header::builder()
    ///     .text("")
    ///     .set_block_id(Some("header_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "header",
    ///     "block_id": "header_1",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Header;
    /// let header = Header::builder()
    ///     .text("")
    ///     .block_id("header_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "header",
    ///     "block_id": "header_1",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(header).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build a [`Header`] object. This method will panic if text is not set.
    pub fn build(self) -> Header {
        Header {
            kind: "header",
            block_id: self.block_id,
            text: self.text.expect("text must be set to HeaderBuilder"),
        }
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<Text> {
        &self.text
    }
}
