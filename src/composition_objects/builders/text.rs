use super::{Text, TYPE_MRKDWN, TYPE_PLAIN};

impl Text {
    /// Construct a [`TextBuilder`].
    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }
}

/// Builder for [`Text`] object.
#[derive(Debug, Default)]
pub struct TextBuilder {
    kind: Option<&'static str>,
    text: Option<String>,
    emoji: Option<bool>,
    verbatim: Option<bool>,
}

impl TextBuilder {
    /// Set plain text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .plain_text("hello world")
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
    pub fn plain_text(self, text: impl Into<String>) -> Self {
        self.set_plain_text(Some(text.into()))
    }

    /// Set plain text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .set_plain_text(Some("hello world".into()))
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
    pub fn set_plain_text(self, text: Option<String>) -> Self {
        Self {
            kind: Some(TYPE_PLAIN),
            text,
            ..self
        }
    }

    /// Set markdown text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .mrkdwn("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn mrkdwn(self, text: impl Into<String>) -> Self {
        self.set_mrkdwn(Some(text.into()))
    }

    /// Set markdown text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .set_mrkdwn(Some("hello world".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_mrkdwn(self, text: Option<String>) -> Self {
        Self {
            kind: Some(TYPE_MRKDWN),
            text,
            ..self
        }
    }

    /// Set emoji field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .plain_text("😊")
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
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .plain_text("😊")
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

    /// Set verbatim field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .mrkdwn("hello world")
    ///     .verbatim(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "mrkdwn",
    ///    "text": "hello world",
    ///    "verbatim": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn verbatim(self, verbatim: bool) -> Self {
        self.set_verbatim(Some(verbatim))
    }

    /// Set verbatim field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::Text;
    /// let text = Text::builder()
    ///     .mrkdwn("hello world")
    ///     .set_verbatim(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "mrkdwn",
    ///    "text": "hello world",
    ///    "verbatim": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_verbatim(self, verbatim: Option<bool>) -> Self {
        Self { verbatim, ..self }
    }

    /// Build a [`Text`] object. This method will panic if either `type` of `text` is not set.
    pub fn build(self) -> Text {
        Text {
            kind: self.kind.expect("text type must be set to TextBuilder"),
            text: self.text.expect("text must be set to TextBuilder"),
            emoji: self.emoji,
            verbatim: self.verbatim,
        }
    }

    /// Get type value.
    pub fn get_type(&self) -> &Option<&'static str> {
        &self.kind
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get emoji value.
    pub fn get_emoji(&self) -> &Option<bool> {
        &self.emoji
    }

    /// Get verbatim value.
    pub fn get_verbatim(&self) -> &Option<bool> {
        &self.verbatim
    }
}
