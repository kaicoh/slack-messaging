use serde::Serialize;

/// Type of conversation to set into [Filter object](https://api.slack.com/reference/block-kit/composition-objects#filter_conversations)
#[derive(Debug, Serialize)]
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
/// use slack_messaging::blocks::elements::{Filter, Conversation};
/// use serde_json::json;
///
/// let filter = Filter::new()
///     .include(Conversation::Public)
///     .include(Conversation::Mpim)
///     .exclude_bot_users();
///
/// let expected = json!({
///     "include": [
///         "public",
///         "mpim"
///     ],
///     "exclude_bot_users": true
/// });
///
/// let filter_json = serde_json::to_value(filter).unwrap();
///
/// assert_eq!(filter_json, expected);
/// ```
#[derive(Debug, Default, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    include: Vec<Conversation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_external_shared_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_bot_users: Option<bool>,
}

impl Filter {
    /// Constructs a Filter object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::Filter;
    /// use serde_json::json;
    ///
    /// let filter = Filter::new();
    ///
    /// let expected = json!({});
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets include field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .set_include(
    ///         vec![
    ///             Conversation::Im,
    ///             Conversation::Private,
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "include": [
    ///         "im",
    ///         "private"
    ///     ]
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn set_include(self, include: Vec<Conversation>) -> Self {
        Self { include, ..self }
    }

    /// Adds conversation to include field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .push_include(Conversation::Mpim);
    ///
    /// let expected = json!({
    ///     "include": [
    ///         "mpim"
    ///     ]
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn push_include(self, conversation: Conversation) -> Self {
        let Self { mut include, .. } = self;
        include.push(conversation);
        Self { include, ..self }
    }

    /// Alias for `push_include` method
    pub fn include(self, conversation: Conversation) -> Self {
        self.push_include(conversation)
    }

    /// Sets exclude_external_shared_channels field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .set_exclude_external_shared_channels(true);
    ///
    /// let expected = json!({
    ///     "exclude_external_shared_channels": true
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn set_exclude_external_shared_channels(self, value: bool) -> Self {
        Self {
            exclude_external_shared_channels: Some(value),
            ..self
        }
    }

    /// Sets true to exclude_external_shared_channels field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .exclude_external_shared_channels();
    ///
    /// let expected = json!({
    ///     "exclude_external_shared_channels": true
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn exclude_external_shared_channels(self) -> Self {
        self.set_exclude_external_shared_channels(true)
    }

    /// Sets exclude_bot_users field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .set_exclude_bot_users(true);
    ///
    /// let expected = json!({
    ///     "exclude_bot_users": true
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn set_exclude_bot_users(self, value: bool) -> Self {
        Self {
            exclude_bot_users: Some(value),
            ..self
        }
    }

    /// Sets true to exclude_bot_users field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let filter = Filter::new()
    ///     .exclude_bot_users();
    ///
    /// let expected = json!({
    ///     "exclude_bot_users": true
    /// });
    ///
    /// let filter_json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(filter_json, expected);
    /// ```
    pub fn exclude_bot_users(self) -> Self {
        self.set_exclude_bot_users(true)
    }
}
