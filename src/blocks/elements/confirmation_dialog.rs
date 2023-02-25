use super::Text;
use serde::Serialize;

/// [Confirmation dialog object](https://api.slack.com/reference/block-kit/composition-objects#confirm)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::ConfirmationDialog;
/// use serde_json::json;
///
/// let confirm = ConfirmationDialog::new()
///     .set_title("Are you sure?")
///     .set_text("Wouldn't you prefer a good game of _chess_?")
///     .set_confirm("Do it")
///     .set_deny("Stop, I've changed my mind!");
///
/// let expected = json!({
///     "title": {
///         "type": "plain_text",
///         "text": "Are you sure?",
///         "emoji": true
///     },
///     "text": {
///         "type": "plain_text",
///         "text": "Wouldn't you prefer a good game of _chess_?",
///         "emoji": true
///     },
///     "confirm": {
///         "type": "plain_text",
///         "text": "Do it",
///         "emoji": true
///     },
///     "deny": {
///         "type": "plain_text",
///         "text": "Stop, I've changed my mind!",
///         "emoji": true
///     }
/// });
///
/// let confirm_json = serde_json::to_value(confirm).unwrap();
///
/// assert_eq!(confirm_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct ConfirmationDialog {
    title: Text,

    text: Text,

    confirm: Text,

    deny: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,
}

impl Default for ConfirmationDialog {
    fn default() -> Self {
        Self {
            title: Text::plain(""),
            text: Text::plain(""),
            confirm: Text::plain(""),
            deny: Text::plain(""),
            style: None,
        }
    }
}

impl ConfirmationDialog {
    /// Constructs a Confirmation dialog object with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new();
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["title"]["text"], Value::String("".into()));
    /// assert_eq!(confirm_json["text"]["text"], Value::String("".into()));
    /// assert_eq!(confirm_json["confirm"]["text"], Value::String("".into()));
    /// assert_eq!(confirm_json["deny"]["text"], Value::String("".into()));
    /// assert_eq!(confirm_json["style"], Value::Null);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets title field with plain_text object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new()
    ///     .set_title("Are you sure?");
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["title"]["text"], Value::String("Are you sure?".into()));
    /// ```
    pub fn set_title<T: Into<String>>(self, title: T) -> Self {
        Self {
            title: Text::plain(title),
            ..self
        }
    }

    /// Sets text field with plain_text object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new()
    ///     .set_text("Wouldn't you prefer a good game of _chess_?");
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["text"]["text"], Value::String("Wouldn't you prefer a good game of _chess_?".into()));
    /// ```
    pub fn set_text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: Text::plain(text),
            ..self
        }
    }

    /// Sets confirm field with plain_text object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new()
    ///     .set_confirm("Do it");
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["confirm"]["text"], Value::String("Do it".into()));
    /// ```
    pub fn set_confirm<T: Into<String>>(self, confirm: T) -> Self {
        Self {
            confirm: Text::plain(confirm),
            ..self
        }
    }

    /// Sets deny field with plain_text object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new()
    ///     .set_deny("Stop, I've changed my mind!");
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["deny"]["text"], Value::String("Stop, I've changed my mind!".into()));
    /// ```
    pub fn set_deny<T: Into<String>>(self, deny: T) -> Self {
        Self {
            deny: Text::plain(deny),
            ..self
        }
    }

    /// Sets style field as "primary".
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new().set_primary();
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["style"], Value::String("primary".into()));
    /// ```
    pub fn set_primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    /// Sets style field as "danger".
    ///
    /// ```
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::{json, Value};
    ///
    /// let confirm = ConfirmationDialog::new().set_danger();
    ///
    /// let confirm_json = serde_json::to_value(confirm).unwrap();
    ///
    /// assert_eq!(confirm_json["style"], Value::String("danger".into()));
    /// ```
    pub fn set_danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }
}
