use super::RichTextElementTypeColor;

impl RichTextElementTypeColor {
    /// Construct a [`RichTextElementTypeColorBuilder`].
    pub fn builder() -> RichTextElementTypeColorBuilder {
        RichTextElementTypeColorBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeColor`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeColorBuilder {
    value: Option<String>,
}

impl RichTextElementTypeColorBuilder {
    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeColor;
    /// let color = RichTextElementTypeColor::builder()
    ///     .set_value(Some("#F405B3".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "color",
    ///     "value": "#F405B3"
    /// });
    ///
    /// let json = serde_json::to_value(color).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self { value }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeColor;
    /// let color = RichTextElementTypeColor::builder()
    ///     .value("#F405B3")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "color",
    ///     "value": "#F405B3"
    /// });
    ///
    /// let json = serde_json::to_value(color).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value.into()))
    }

    /// Build a [`RichTextElementTypeColor`] object. This method will panic if value is not
    /// set.
    pub fn build(self) -> RichTextElementTypeColor {
        RichTextElementTypeColor {
            kind: "color",
            value: self
                .value
                .expect("value must be set to RichTextElementTypeColorBuilder"),
        }
    }

    /// Get color value.
    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }
}
