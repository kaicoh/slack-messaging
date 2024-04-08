use super::{CodableStyle, RichTextElementTypeText};

impl RichTextElementTypeText {
    /// Construct a [`RichTextElementTypeTextBuilder`].
    pub fn builder() -> RichTextElementTypeTextBuilder {
        RichTextElementTypeTextBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeText`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeTextBuilder {
    text: Option<String>,
    style: Option<CodableStyle>,
}

impl RichTextElementTypeTextBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeText;
    /// let text = RichTextElementTypeText::builder()
    ///     .set_text(Some("hello".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "text",
    ///     "text": "hello"
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeText;
    /// let text = RichTextElementTypeText::builder()
    ///     .text("hello")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "text",
    ///     "text": "hello"
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeText,
    /// CodableStyle};
    /// let text = RichTextElementTypeText::builder()
    ///     .text("")
    ///     .set_style(
    ///         Some(CodableStyle::builder()
    ///             .bold(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "text",
    ///     "text": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<CodableStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeText,
    /// CodableStyle};
    /// let text = RichTextElementTypeText::builder()
    ///     .text("")
    ///     .style(
    ///         CodableStyle::builder()
    ///             .bold(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "text",
    ///     "text": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: CodableStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Build a [`RichTextElementTypeText`] object. This method will panic if text is not
    /// set.
    pub fn build(self) -> RichTextElementTypeText {
        RichTextElementTypeText {
            kind: "text",
            text: self
                .text
                .expect("text must be set to RichTextElementTypeTextBuilder"),
            style: self.style,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<String> {
        &self.text
    }

    /// Get style value.
    pub fn get_style(&self) -> &Option<CodableStyle> {
        &self.style
    }
}
