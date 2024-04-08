use super::{Conversation, ConversationFilter};

impl ConversationFilter {
    /// Construct a [`ConversationFilterBuilder`].
    pub fn builder() -> ConversationFilterBuilder {
        ConversationFilterBuilder::default()
    }
}

/// Builder for [`Filter`] object.
#[derive(Debug, Default)]
pub struct ConversationFilterBuilder {
    include: Vec<Conversation>,
    exclude_external_shared_channels: Option<bool>,
    exclude_bot_users: Option<bool>,
}

impl ConversationFilterBuilder {
    /// Set include field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .set_include(
    ///         vec![
    ///             Conversation::Im,
    ///             Conversation::Private,
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "include": [
    ///         "im",
    ///         "private"
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_include(self, include: Vec<Conversation>) -> Self {
        Self { include, ..self }
    }

    /// Add conversation to include field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .include(Conversation::Mpim)
    ///     .include(Conversation::Public)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "include": [
    ///         "mpim",
    ///         "public"
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn include(self, conversation: Conversation) -> Self {
        let Self { mut include, .. } = self;
        include.push(conversation);
        Self { include, ..self }
    }

    /// Set exclude_external_shared_channels field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .set_exclude_external_shared_channels(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_external_shared_channels": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_exclude_external_shared_channels(self, value: Option<bool>) -> Self {
        Self {
            exclude_external_shared_channels: value,
            ..self
        }
    }

    /// Set exclude_external_shared_channels field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .exclude_external_shared_channels(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_external_shared_channels": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn exclude_external_shared_channels(self, value: bool) -> Self {
        self.set_exclude_external_shared_channels(Some(value))
    }

    /// Set exclude_bot_users field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .set_exclude_bot_users(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_bot_users": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_exclude_bot_users(self, value: Option<bool>) -> Self {
        Self {
            exclude_bot_users: value,
            ..self
        }
    }

    /// Set exclude_bot_users field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// let filter = ConversationFilter::builder()
    ///     .exclude_bot_users(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_bot_users": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn exclude_bot_users(self, value: bool) -> Self {
        self.set_exclude_bot_users(Some(value))
    }

    /// Build a [`ConversationFilter`] object.
    pub fn build(self) -> ConversationFilter {
        ConversationFilter {
            include: self.include,
            exclude_external_shared_channels: self.exclude_external_shared_channels,
            exclude_bot_users: self.exclude_bot_users,
        }
    }
}
