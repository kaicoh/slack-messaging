use super::{ConfirmationDialog, Filter, Text};
use serde::Serialize;

/// [Multi-select menu Conversations list element](https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectConversations;
/// let menu = MultiSelectConversations::builder()
///     .action_id("text1234")
///     .placeholder("Select conversations")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select conversations"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectConversations {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

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

impl MultiSelectConversations {
    /// Construct a [`MultiSelectConversationsBuilder`].
    pub fn builder() -> MultiSelectConversationsBuilder {
        MultiSelectConversationsBuilder::default()
    }
}

/// Builder for [`MultiSelectConversations`] object.
#[derive(Debug, Default)]
pub struct MultiSelectConversationsBuilder {
    action_id: Option<String>,
    initial_conversations: Vec<String>,
    default_to_current_conversation: Option<bool>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    filter: Option<Filter>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl MultiSelectConversationsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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

    /// Set initial_conversations field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_initial_conversations(
    ///         vec!["conversation_0".to_string(), "conversation_1".to_string()]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "initial_conversations": ["conversation_0", "conversation_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_conversations(self, initial_conversations: Vec<String>) -> Self {
        Self {
            initial_conversations,
            ..self
        }
    }

    /// Add conversation id to initial_conversations field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .initial_conversation("conversation_0")
    ///     .initial_conversation("conversation_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "initial_conversations": ["conversation_0", "conversation_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_conversation(self, conversation: impl Into<String>) -> Self {
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

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_default_to_current_conversation(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_default_to_current_conversation(self, current_conversation: Option<bool>) -> Self {
        Self {
            default_to_current_conversation: current_conversation,
            ..self
        }
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .default_to_current_conversation(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn default_to_current_conversation(self, current_conversation: bool) -> Self {
        self.set_default_to_current_conversation(Some(current_conversation))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{MultiSelectConversations, ConfirmationDialog};
    /// let menu = MultiSelectConversations::builder()
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
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::{MultiSelectConversations, ConfirmationDialog};
    /// let menu = MultiSelectConversations::builder()
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
    ///     "type": "multi_conversations_select",
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

    /// Set max_selected_items field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_max_selected_items(Some(3))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_selected_items(self, items: Option<i64>) -> Self {
        Self {
            max_selected_items: items,
            ..self
        }
    }

    /// Set max_selected_items field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .max_selected_items(3)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_selected_items(self, items: impl Into<i64>) -> Self {
        self.set_max_selected_items(Some(items.into()))
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{MultiSelectConversations, Filter, Conversation};
    /// let menu = MultiSelectConversations::builder()
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
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::{MultiSelectConversations, Filter, Conversation};
    /// let menu = MultiSelectConversations::builder()
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
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_placeholder(Some(plain_text!("Select conversations")))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select conversations"
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// let menu = MultiSelectConversations::builder()
    ///     .placeholder("Select conversations")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select conversations"
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

    /// Build a [`MultiSelectConversations`] object.
    pub fn build(self) -> MultiSelectConversations {
        MultiSelectConversations {
            kind: "multi_conversations_select",
            action_id: self.action_id,
            initial_conversations: self.initial_conversations,
            default_to_current_conversation: self.default_to_current_conversation,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            filter: self.filter,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }
}
