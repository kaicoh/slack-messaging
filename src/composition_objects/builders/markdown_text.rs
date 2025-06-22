use super::MrkdwnText;

impl MrkdwnText {
    /// Construct a [`MrkdwnTextBuilder`].
    pub fn builder() -> MrkdwnTextBuilder {
        MrkdwnTextBuilder::default()
    }
}

/// Builder for [`Text`] object.
#[derive(Debug, Default)]
pub struct MrkdwnTextBuilder {
    text: Option<String>,
    verbatim: Option<bool>,
}

impl MrkdwnTextBuilder {
    /// Set text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let text = MrkdwnText::builder()
    ///     .text("hello world")
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
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set text.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let text = MrkdwnText::builder()
    ///     .set_text(Some("hello world".into()))
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
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set verbatim field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let text = MrkdwnText::builder()
    ///     .text("hello world")
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
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let text = MrkdwnText::builder()
    ///     .text("😊")
    ///     .set_verbatim(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "mrkdwn",
    ///    "text": "😊",
    ///    "verbatim": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_verbatim(self, verbatim: Option<bool>) -> Self {
        Self { verbatim, ..self }
    }

    /// Build a [`MrkdwnText`] object. This method will panic if `text` field is not set.
    pub fn build(self) -> MrkdwnText {
        MrkdwnText {
            text: self.text.expect("text must be set to MrkdwnTextBuilder"),
            verbatim: self.verbatim,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get verbatim value.
    pub fn get_verbatim(&self) -> &Option<bool> {
        &self.verbatim
    }
}
