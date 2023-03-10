use super::{ConfirmationDialog, Filter, Text};
use serde::Serialize;

/// [Select menu of conversations element](https://api.slack.com/reference/block-kit/block-elements#conversations_select)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::SelectConversations;
/// use serde_json::json;
///
/// let menu = SelectConversations::new()
///     .set_action_id("text1234")
///     .placeholder("Select an item");
///
/// let expected = json!({
///     "type": "conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SelectConversations {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

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

impl Default for SelectConversations {
    fn default() -> Self {
        Self {
            kind: "conversations_select",
            action_id: "".into(),
            initial_conversation: None,
            default_to_current_conversation: None,
            confirm: None,
            response_url_enabled: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl SelectConversations {
    /// Constructs a Select menu of conversations element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new();
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": ""
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_conversation field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new()
    ///     .set_initial_conversation("conversation_000");
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "initial_conversation": "conversation_000"
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_initial_conversation<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_conversation: Some(value.into()),
            ..self
        }
    }

    /// Sets default_to_current_conversation field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new()
    ///     .set_default_to_current_conversation(true);
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_default_to_current_conversation(self, current_conversation: bool) -> Self {
        Self {
            default_to_current_conversation: Some(current_conversation),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectConversations, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?",
    ///             "emoji": true
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?",
    ///             "emoji": true
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it",
    ///             "emoji": true
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!",
    ///             "emoji": true
    ///         }
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets response_url_enabled field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().set_response_url_enabled(true);
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_response_url_enabled(self, enabled: bool) -> Self {
        Self {
            response_url_enabled: Some(enabled),
            ..self
        }
    }

    /// Sets true to response_url_enabled field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().response_url_enabled();
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn response_url_enabled(self) -> Self {
        self.set_response_url_enabled(true)
    }

    /// Sets false to response_url_enabled field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().response_url_disabled();
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "response_url_enabled": false
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn response_url_disabled(self) -> Self {
        self.set_response_url_enabled(false)
    }

    /// Sets filter field with Filter object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectConversations, Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new()
    ///     .set_filter(
    ///         Filter::new()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users()
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "filter": {
    ///         "include": [
    ///             "public",
    ///             "mpim"
    ///         ],
    ///         "exclude_bot_users": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_filter(self, filter: Filter) -> Self {
        Self {
            filter: Some(filter),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }

    /// Sets placeholder field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectConversations, Text};
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new()
    ///     .set_placeholder(Text::plain("Select an item"));
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    /// Sets placeholder field from string. This is a shorthand for `set_placeholder` method.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = SelectConversations::new().placeholder("Select an item");
    ///
    /// let expected = json!({
    ///     "type": "conversations_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }
}
