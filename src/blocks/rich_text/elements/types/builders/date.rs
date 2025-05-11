use super::RichTextElementTypeDate;

impl RichTextElementTypeDate {
    /// Construct a [`RichTextElementTypeDateBuilder`].
    pub fn builder() -> RichTextElementTypeDateBuilder {
        RichTextElementTypeDateBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeDate`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeDateBuilder {
    timestamp: Option<i64>,
    format: Option<String>,
    link: Option<String>,
    fallback: Option<String>,
}

impl RichTextElementTypeDateBuilder {
    /// Set timestamp field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_timestamp(self, timestamp: Option<i64>) -> Self {
        Self { timestamp, ..self }
    }

    /// Set timestamp field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .timestamp(1720710212)
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn timestamp(self, timestamp: i64) -> Self {
        self.set_timestamp(Some(timestamp))
    }

    /// Set format field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_format(self, format: Option<String>) -> Self {
        Self { format, ..self }
    }

    /// Set format field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .format("{date_num} at {time}")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn format(self, format: impl Into<String>) -> Self {
        self.set_format(Some(format.into()))
    }

    /// Set link field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .set_link(Some("https://example.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}",
    ///     "link": "https://example.com"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_link(self, link: Option<String>) -> Self {
        Self { link, ..self }
    }

    /// Set link field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .link("https://example.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}",
    ///     "link": "https://example.com"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn link(self, link: impl Into<String>) -> Self {
        self.set_link(Some(link.into()))
    }

    /// Set fallback field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .set_fallback(Some("timey".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}",
    ///     "fallback": "timey"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_fallback(self, fallback: Option<String>) -> Self {
        Self { fallback, ..self }
    }

    /// Set fallback field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeDate;
    /// let date = RichTextElementTypeDate::builder()
    ///     .set_timestamp(Some(1720710212))
    ///     .set_format(Some("{date_num} at {time}".into()))
    ///     .fallback("timey")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "date",
    ///     "timestamp": 1720710212,
    ///     "format": "{date_num} at {time}",
    ///     "fallback": "timey"
    /// });
    ///
    /// let json = serde_json::to_value(date).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn fallback(self, fallback: impl Into<String>) -> Self {
        self.set_fallback(Some(fallback.into()))
    }

    /// Build a [`RichTextElementTypeDate`] object. This method will panic if either timestamp
    /// or format is not set.
    pub fn build(self) -> RichTextElementTypeDate {
        RichTextElementTypeDate {
            kind: "date",
            timestamp: self
                .timestamp
                .expect("timestamp must be set to RichTextElementTypeDateBuilder"),
            format: self
                .format
                .expect("format must be set to RichTextElementTypeDateBuilder"),
            link: self.link,
            fallback: self.fallback,
        }
    }

    /// Get timestamp value.
    pub fn get_timestamp(&self) -> &Option<i64> {
        &self.timestamp
    }

    /// Get format value.
    pub fn get_format(&self) -> &Option<String> {
        &self.format
    }

    /// Get link value.
    pub fn get_link(&self) -> &Option<String> {
        &self.link
    }

    /// Get fallback value.
    pub fn get_fallback(&self) -> &Option<String> {
        &self.fallback
    }
}
