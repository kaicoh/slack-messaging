use super::RichTextElementTypeEmoji;

impl RichTextElementTypeEmoji {
    /// Construct a [`RichTextElementTypeEmojiBuilder`].
    pub fn builder() -> RichTextElementTypeEmojiBuilder {
        RichTextElementTypeEmojiBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeEmoji`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeEmojiBuilder {
    name: Option<String>,
    unicode: Option<String>,
}

impl RichTextElementTypeEmojiBuilder {
    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeEmoji;
    /// let emoji = RichTextElementTypeEmoji::builder()
    ///     .set_name(Some("emoji-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "emoji",
    ///     "name": "emoji-0"
    /// });
    ///
    /// let json = serde_json::to_value(emoji).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_name(self, name: Option<String>) -> Self {
        Self { name, ..self }
    }

    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeEmoji;
    /// let emoji = RichTextElementTypeEmoji::builder()
    ///     .name("emoji-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "emoji",
    ///     "name": "emoji-0"
    /// });
    ///
    /// let json = serde_json::to_value(emoji).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn name(self, name: impl Into<String>) -> Self {
        self.set_name(Some(name.into()))
    }

    /// Set unicode field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeEmoji;
    /// let emoji = RichTextElementTypeEmoji::builder()
    ///     .set_name(Some("emoji-0".into()))
    ///     .set_unicode(Some("unicode-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "emoji",
    ///     "name": "emoji-0",
    ///     "unicode": "unicode-0",
    /// });
    ///
    /// let json = serde_json::to_value(emoji).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_unicode(self, unicode: Option<String>) -> Self {
        Self { unicode, ..self }
    }

    /// Set unicode field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeEmoji;
    /// let emoji = RichTextElementTypeEmoji::builder()
    ///     .name("emoji-0")
    ///     .unicode("unicode-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "emoji",
    ///     "name": "emoji-0",
    ///     "unicode": "unicode-0",
    /// });
    ///
    /// let json = serde_json::to_value(emoji).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn unicode(self, unicode: impl Into<String>) -> Self {
        self.set_unicode(Some(unicode.into()))
    }

    /// Build a [`RichTextElementTypeEmoji`] object. This method will panic if name is not
    /// set.
    pub fn build(self) -> RichTextElementTypeEmoji {
        RichTextElementTypeEmoji {
            kind: "emoji",
            name: self
                .name
                .expect("name must be set to RichTextElementTypeEmojiBuilder"),
            unicode: self.unicode,
        }
    }

    /// Get name value.
    pub fn get_name(&self) -> &Option<String> {
        &self.name
    }

    /// Get unicode value.
    pub fn get_unicode(&self) -> &Option<String> {
        &self.unicode
    }
}
