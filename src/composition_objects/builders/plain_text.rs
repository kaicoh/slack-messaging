use super::PlainText;

impl PlainText {
    /// Construct a [`PlainTextBuilder`].
    pub fn builder() -> PlainTextBuilder {
        PlainTextBuilder::default()
    }
}

/// Builder for [`Text`] object.
#[derive(Debug, Default)]
pub struct PlainTextBuilder {
    text: Option<String>,
    emoji: Option<bool>,
}

impl PlainTextBuilder {
    /// Set text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::PlainText;
    /// let text = PlainText::builder()
    ///     .text("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::PlainText;
    /// let text = PlainText::builder()
    ///     .set_text(Some("hello world".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set emoji field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::PlainText;
    /// let text = PlainText::builder()
    ///     .text("😊")
    ///     .emoji(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "plain_text",
    ///    "text": "😊",
    ///    "emoji": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn emoji(self, emoji: bool) -> Self {
        self.set_emoji(Some(emoji))
    }

    /// Set emoji field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::PlainText;
    /// let text = PlainText::builder()
    ///     .text("😊")
    ///     .set_emoji(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "plain_text",
    ///    "text": "😊",
    ///    "emoji": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_emoji(self, emoji: Option<bool>) -> Self {
        Self { emoji, ..self }
    }

    /// Build a [`PlainText`] object. This method will panic if `text` field is not set.
    pub fn build(self) -> PlainText {
        PlainText {
            text: self.text.expect("text must be set to TextBuilder"),
            emoji: self.emoji,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get emoji value.
    pub fn get_emoji(&self) -> &Option<bool> {
        &self.emoji
    }
}
