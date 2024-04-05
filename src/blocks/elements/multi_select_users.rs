use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Multi-select menu User list element](https://api.slack.com/reference/block-kit/block-elements#users_multi_select)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::MultiSelectUsers;
/// use serde_json::json;
///
/// let menu = MultiSelectUsers::new()
///     .set_action_id("text1234")
///     .push_initial_user("user9999")
///     .placeholder("Select users");
///
/// let expected = json!({
///     "type": "multi_users_select",
///     "action_id": "text1234",
///     "initial_users": [
///         "user9999"
///     ],
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select users",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectUsers {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_users: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for MultiSelectUsers {
    fn default() -> Self {
        Self {
            kind: "multi_users_select",
            action_id: "".into(),
            initial_users: vec![],
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl MultiSelectUsers {
    /// Constructs a Multi-select menu User list element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new();
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
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
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
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

    /// Sets initial_users field directly.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new()
    ///     .set_initial_users(
    ///         vec!["user0000".into(), "user9999".into()]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
    ///     "action_id": "",
    ///     "initial_users": ["user0000", "user9999"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_initial_users(self, initial_users: Vec<String>) -> Self {
        Self {
            initial_users,
            ..self
        }
    }

    /// Adds string to initial_users field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new()
    ///     .push_initial_user("user0000");
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
    ///     "action_id": "",
    ///     "initial_users": ["user0000"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn push_initial_user<T: Into<String>>(self, user: T) -> Self {
        let Self {
            mut initial_users, ..
        } = self;
        initial_users.push(user.into());
        Self {
            initial_users,
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectUsers, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
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
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new().set_max_selected_items(3);
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
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

    /// Sets focus_on_load field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
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
    /// use slack_messaging::blocks::elements::{MultiSelectUsers, Text};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectUsers::new().set_placeholder(Text::plain("Select users"));
    ///
    /// let expected = json!({
    ///     "type": "multi_users_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select users",
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
