use super::{
    MultiSelectUsers,
    composition_objects::{ConfirmationDialog, Text},
};

impl MultiSelectUsers {
    /// Construct a [`MultiSelectUsersBuilder`].
    pub fn builder() -> MultiSelectUsersBuilder {
        MultiSelectUsersBuilder::default()
    }
}

/// Builder for [`MultiSelectUsers`] object.
#[derive(Debug, Default)]
pub struct MultiSelectUsersBuilder {
    action_id: Option<String>,
    initial_users: Vec<String>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl MultiSelectUsersBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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

    /// Set initial_users field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .set_initial_users(
    ///         vec!["user0000".into(), "user9999".into()]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "initial_users": ["user0000", "user9999"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_users(self, initial_users: Vec<String>) -> Self {
        Self {
            initial_users,
            ..self
        }
    }

    /// Add user id to initial_users field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .initial_user("user0000")
    ///     .initial_user("user0001")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "initial_users": ["user0000", "user0001"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_user(self, user: impl Into<String>) -> Self {
        let Self {
            mut initial_users, ..
        } = self;
        initial_users.push(user.into());
        Self {
            initial_users,
            ..self
        }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectUsers::builder()
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
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectUsers::builder()
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
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .set_max_selected_items(Some(30))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "max_selected_items": 30
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .max_selected_items(30)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "max_selected_items": 30
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_selected_items(self, items: impl Into<i64>) -> Self {
        self.set_max_selected_items(Some(items.into()))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = MultiSelectUsers::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select users")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select users"
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
    /// # use slack_messaging::blocks::elements::MultiSelectUsers;
    /// let menu = MultiSelectUsers::builder()
    ///     .placeholder("Select users")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select users"
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

    /// Build a [`MultiSelectUsers`] object.
    pub fn build(self) -> MultiSelectUsers {
        MultiSelectUsers {
            kind: "multi_users_select",
            action_id: self.action_id,
            initial_users: self.initial_users,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_users value.
    pub fn get_initial_users(&self) -> &[String] {
        &self.initial_users
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get max_selected_items value.
    pub fn get_max_selected_items(&self) -> &Option<i64> {
        &self.max_selected_items
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
