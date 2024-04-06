use serde::Serialize;

/// Type of conversation to set into [Filter object](https://api.slack.com/reference/block-kit/composition-objects#filter_conversations)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Conversation {
    Im,
    Mpim,
    Private,
    Public,
}

/// [Filter object](https://api.slack.com/reference/block-kit/composition-objects#filter_conversations)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{Filter, Conversation};
/// let filter = Filter::builder()
///     .include(Conversation::Public)
///     .include(Conversation::Mpim)
///     .exclude_bot_users(true)
///     .build();
///
/// let expected = serde_json::json!({
///     "include": [
///         "public",
///         "mpim"
///     ],
///     "exclude_bot_users": true
/// });
///
/// let json = serde_json::to_value(filter).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    include: Vec<Conversation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_external_shared_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_bot_users: Option<bool>,
}

impl Filter {
    /// Construct a [`FilterBuilder`].
    pub fn builder() -> FilterBuilder {
        FilterBuilder::default()
    }
}

/// Builder for [`Filter`] object.
#[derive(Debug, Default)]
pub struct FilterBuilder {
    include: Vec<Conversation>,
    exclude_external_shared_channels: Option<bool>,
    exclude_bot_users: Option<bool>,
}

impl FilterBuilder {
    /// Set include field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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
    /// # use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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
    /// # use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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
    /// # use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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
    /// # use slack_messaging::blocks::elements::{Filter, Conversation};
    /// let filter = Filter::builder()
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

    /// Build a [`Filter`] object.
    pub fn build(self) -> Filter {
        Filter {
            include: self.include,
            exclude_external_shared_channels: self.exclude_external_shared_channels,
            exclude_bot_users: self.exclude_bot_users,
        }
    }
}
