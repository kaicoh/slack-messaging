use serde::Serialize;

/// [**emoji**](https://api.slack.com/reference/block-kit/blocks#emoji-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeEmoji;
/// let emoji = RichTextElementTypeEmoji::builder()
///     .name("wave")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "emoji",
///     "name": "wave"
/// });
///
/// let json = serde_json::to_value(emoji).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeEmoji {
    #[serde(rename = "type")]
    kind: &'static str,
    name: String,
}

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
}

impl RichTextElementTypeEmojiBuilder {
    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeEmoji;
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
        Self { name }
    }

    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeEmoji;
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

    /// Build a [`RichTextElementTypeEmoji`] object. This method will panic if name is not
    /// set.
    pub fn build(self) -> RichTextElementTypeEmoji {
        RichTextElementTypeEmoji {
            kind: "emoji",
            name: self
                .name
                .expect("name must be set to RichTextElementTypeEmojiBuilder"),
        }
    }
}