use super::{ConfirmationDialog, Filter, Text};
use serde::Serialize;

/// [Select menu of conversations element](https://api.slack.com/reference/block-kit/block-elements#conversations_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectConversations;
/// let menu = SelectConversations::builder()
///     .action_id("text1234")
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SelectConversations {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_url_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl SelectConversations {
    /// Construct a [`SelectConversationsBuilder`].
    pub fn builder() -> SelectConversationsBuilder {
        SelectConversationsBuilder::default()
    }
}

/// Builder for [`SelectConversations`] object.
#[derive(Debug, Default)]
pub struct SelectConversationsBuilder {
    action_id: Option<String>,
    initial_conversation: Option<String>,
    default_to_current_conversation: Option<bool>,
    confirm: Option<ConfirmationDialog>,
    response_url_enabled: Option<bool>,
    filter: Option<Filter>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl SelectConversationsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_initial_conversation(Some("conversation_000".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "initial_conversation": "conversation_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_conversation(self, value: Option<String>) -> Self {
        Self {
            initial_conversation: value,
            ..self
        }
    }

    /// Set initial_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .initial_conversation("conversation_000")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "initial_conversation": "conversation_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_conversation(self, value: impl Into<String>) -> Self {
        self.set_initial_conversation(Some(value.into()))
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_default_to_current_conversation(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_default_to_current_conversation(self, value: Option<bool>) -> Self {
        Self {
            default_to_current_conversation: value,
            ..self
        }
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .default_to_current_conversation(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn default_to_current_conversation(self, value: bool) -> Self {
        self.set_default_to_current_conversation(Some(value))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{SelectConversations, ConfirmationDialog};
    /// let menu = SelectConversations::builder()
    ///     .set_confirm(
    ///         Some(ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{SelectConversations, ConfirmationDialog};
    /// let menu = SelectConversations::builder()
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set response_url_enabled field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_response_url_enabled(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_response_url_enabled(self, enabled: Option<bool>) -> Self {
        Self {
            response_url_enabled: enabled,
            ..self
        }
    }

    /// Set response_url_enabled field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .response_url_enabled(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn response_url_enabled(self, enabled: bool) -> Self {
        self.set_response_url_enabled(Some(enabled))
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{SelectConversations, Filter, Conversation};
    /// let menu = SelectConversations::builder()
    ///     .set_filter(
    ///         Some(Filter::builder()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "filter": {
    ///         "include": [
    ///             "public",
    ///             "mpim"
    ///         ],
    ///         "exclude_bot_users": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_filter(self, filter: Option<Filter>) -> Self {
        Self { filter, ..self }
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{SelectConversations, Filter, Conversation};
    /// let menu = SelectConversations::builder()
    ///     .filter(
    ///         Filter::builder()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "filter": {
    ///         "include": [
    ///             "public",
    ///             "mpim"
    ///         ],
    ///         "exclude_bot_users": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn filter(self, filter: Filter) -> Self {
        self.set_filter(Some(filter))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: Option<bool>) -> Self {
        Self {
            focus_on_load,
            ..self
        }
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_placeholder(Some(plain_text!("Select an item")))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Option<Text>) -> Self {
        Self {
            placeholder,
            ..self
        }
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .placeholder("Select an item")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`SelectConversations`] object.
    pub fn build(self) -> SelectConversations {
        SelectConversations {
            kind: "conversations_select",
            action_id: self.action_id,
            initial_conversation: self.initial_conversation,
            default_to_current_conversation: self.default_to_current_conversation,
            confirm: self.confirm,
            response_url_enabled: self.response_url_enabled,
            filter: self.filter,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }
}
