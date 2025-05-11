use super::{
    MultiSelectConversations,
    composition_objects::{ConfirmationDialog, ConversationFilter, Text},
};

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
    filter: Option<ConversationFilter>,
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = MultiSelectConversations::builder()
    ///     .set_filter(
    ///         Some(ConversationFilter::builder()
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
    pub fn set_filter(self, filter: Option<ConversationFilter>) -> Self {
        Self { filter, ..self }
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = MultiSelectConversations::builder()
    ///     .filter(
    ///         ConversationFilter::builder()
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
    pub fn filter(self, filter: ConversationFilter) -> Self {
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
    /// # use slack_messaging::blocks::elements::MultiSelectConversations;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = MultiSelectConversations::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select conversations")
    ///             .build())
    ///     )
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

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_conversations value.
    pub fn get_initial_conversations(&self) -> &[String] {
        &self.initial_conversations
    }

    /// Get default_to_current_conversation value.
    pub fn get_default_to_current_conversation(&self) -> &Option<bool> {
        &self.default_to_current_conversation
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get max_selected_items value.
    pub fn get_max_selected_items(&self) -> &Option<i64> {
        &self.max_selected_items
    }

    /// Get filter value.
    pub fn get_filter(&self) -> &Option<ConversationFilter> {
        &self.filter
    }

    /// Get focus_on_load value.
    pub fn get_focus_on_load(&self) -> &Option<bool> {
        &self.focus_on_load
    }

    /// Get action_id value.
    pub fn get_placeholder(&self) -> &Option<Text> {
        &self.placeholder
    }
}
