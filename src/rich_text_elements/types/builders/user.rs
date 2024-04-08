use super::{HighlightableStyle, RichTextElementTypeUser};

impl RichTextElementTypeUser {
    /// Construct a [`RichTextElementTypeUserBuilder`].
    pub fn builder() -> RichTextElementTypeUserBuilder {
        RichTextElementTypeUserBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeUser`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeUserBuilder {
    user_id: Option<String>,
    style: Option<HighlightableStyle>,
}

impl RichTextElementTypeUserBuilder {
    /// Set user_id field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeUser;
    /// let user = RichTextElementTypeUser::builder()
    ///     .set_user_id(Some("user-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "user",
    ///     "user_id": "user-0"
    /// });
    ///
    /// let json = serde_json::to_value(user).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_user_id(self, user_id: Option<String>) -> Self {
        Self { user_id, ..self }
    }

    /// Set user_id field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::RichTextElementTypeUser;
    /// let user = RichTextElementTypeUser::builder()
    ///     .user_id("user-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "user",
    ///     "user_id": "user-0"
    /// });
    ///
    /// let json = serde_json::to_value(user).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn user_id(self, user_id: impl Into<String>) -> Self {
        self.set_user_id(Some(user_id.into()))
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeUser,
    /// HighlightableStyle};
    /// let user = RichTextElementTypeUser::builder()
    ///     .user_id("")
    ///     .set_style(
    ///         Some(HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "user",
    ///     "user_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(user).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<HighlightableStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::types::{RichTextElementTypeUser,
    /// HighlightableStyle};
    /// let user = RichTextElementTypeUser::builder()
    ///     .user_id("")
    ///     .style(
    ///         HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "user",
    ///     "user_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(user).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: HighlightableStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Build a [`RichTextElementTypeUser`] object. This method will panic if user_id is not
    /// set.
    pub fn build(self) -> RichTextElementTypeUser {
        RichTextElementTypeUser {
            kind: "user",
            user_id: self
                .user_id
                .expect("user_id must be set to RichTextElementTypeUserBuilder"),
            style: self.style,
        }
    }

    /// Get user_id value.
    pub fn get_user_id(&self) -> &Option<String> {
        &self.user_id
    }

    /// Get style value.
    pub fn get_style(&self) -> &Option<HighlightableStyle> {
        &self.style
    }
}
