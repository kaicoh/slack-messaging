use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Select menu of users element](https://api.slack.com/reference/block-kit/block-elements#users_select)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::SelectUsers;
/// use serde_json::json;
///
/// let menu = SelectUsers::new()
///     .set_action_id("text1234")
///     .placeholder("Select an item");
///
/// let expected = json!({
///     "type": "users_select",
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
pub struct SelectUsers {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for SelectUsers {
    fn default() -> Self {
        Self {
            kind: "users_select",
            action_id: "".into(),
            initial_user: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl SelectUsers {
    /// Constructs a Select menu of users element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new();
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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
    /// use slack_messaging::blocks::elements::SelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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

    /// Sets initial_user field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new().set_initial_user("user_000");
    ///
    /// let expected = json!({
    ///     "type": "users_select",
    ///     "action_id": "",
    ///     "initial_user": "user_000"
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_initial_user<T: Into<String>>(self, initial_user: T) -> Self {
        Self {
            initial_user: Some(initial_user.into()),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectUsers, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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
    /// use slack_messaging::blocks::elements::{SelectUsers, Text};
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new()
    ///     .set_placeholder(Text::plain("Select an item"));
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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
    /// use slack_messaging::blocks::elements::SelectUsers;
    /// use serde_json::json;
    ///
    /// let menu = SelectUsers::new().placeholder("Select an item");
    ///
    /// let expected = json!({
    ///     "type": "users_select",
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
