use super::{ConfirmationDialog, Filter, Text};
use serde::Serialize;

/// [Multi-select menu Conversations list element](https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::MultiSelectConversations;
/// use serde_json::json;
///
/// let menu = MultiSelectConversations::new()
///     .set_action_id("text1234")
///     .placeholder("Select conversations");
///
/// let expected = json!({
///     "type": "multi_conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select conversations",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectConversations {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_conversations: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for MultiSelectConversations {
    fn default() -> Self {
        Self {
            kind: "multi_conversations_select",
            action_id: "".into(),
            initial_conversations: vec![],
            default_to_current_conversation: None,
            confirm: None,
            max_selected_items: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl MultiSelectConversations {
    /// Constructs a Multi-select menu Conversations list element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new();
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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

    /// Sets initial_conversations field directly.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_initial_conversations(
    ///         vec!["conversation_0".to_string(), "conversation_1".to_string()]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
    ///     "action_id": "",
    ///     "initial_conversations": ["conversation_0", "conversation_1"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_initial_conversations(self, initial_conversations: Vec<String>) -> Self {
        Self {
            initial_conversations,
            ..self
        }
    }

    /// Adds string to initial_conversations field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .push_initial_conversation("conversation_0");
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
    ///     "action_id": "",
    ///     "initial_conversations": ["conversation_0"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn push_initial_conversation<T: Into<String>>(self, conversation: T) -> Self {
        let Self {
            mut initial_conversations,
            ..
        } = self;
        initial_conversations.push(conversation.into());
        Self {
            initial_conversations,
            ..self
        }
    }

    /// Sets default_to_current_conversation field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_default_to_current_conversation(true);
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectConversations, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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

    /// Sets max_selected_items field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_max_selected_items(3);
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
    ///     "action_id": "",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_max_selected_items<T: Into<i64>>(self, items: T) -> Self {
        Self {
            max_selected_items: Some(items.into()),
            ..self
        }
    }

    /// Sets filter field with Filter object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectConversations, Filter, Conversation};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_filter(
    ///         Filter::new()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users()
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectConversations;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
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
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectConversations, Text};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectConversations::new()
    ///     .set_placeholder(Text::plain("Select conversations"));
    ///
    /// let expected = json!({
    ///     "type": "multi_conversations_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select conversations",
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
}
