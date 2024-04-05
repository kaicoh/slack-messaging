use super::{ConfirmationDialog, Text};
use crate::plain_text;
use serde::Serialize;

/// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::Button;
/// use serde_json::json;
///
/// let button = Button::new()
///     .text("Click Me")
///     .set_value("click_me_123")
///     .set_action_id("button-0");
///
/// let expected = json!({
///     "type": "button",
///     "text": {
///         "type": "plain_text",
///         "text": "Click Me",
///         "emoji": true
///     },
///     "value": "click_me_123",
///     "action_id": "button-0"
/// });
///
/// let button_json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(button_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    kind: &'static str,

    text: Text,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_label: Option<String>,
}

impl Default for Button {
    fn default() -> Self {
        Self {
            kind: "button",
            text: plain_text!(""),
            action_id: "".into(),
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
        }
    }
}

impl Button {
    /// Constructs a Button element.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new();
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": ""
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{Button, Text};
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_text(Text::plain("Click Me"));
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me",
    ///         "emoji": true
    ///     },
    ///     "action_id": ""
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    /// Sets action_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_action_id("click_me_123");
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "click_me_123"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_url("https://google.com");
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }

    /// Sets value field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_value("value-0");
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "",
    ///     "value": "value-0"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: Some(value.into()),
            ..self
        }
    }

    /// Sets primary to style field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_primary();
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "",
    ///     "style": "primary"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    /// Sets danger to style field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_danger();
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "",
    ///     "style": "danger"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{Button, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let button = Button::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
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
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets accessibility_label field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let button = Button::new().set_accessibility_label("Click Me");
    ///
    /// let expected = json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "action_id": "",
    ///     "accessibility_label": "Click Me"
    /// });
    ///
    /// let button_json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(button_json, expected);
    /// ```
    pub fn set_accessibility_label<T: Into<String>>(self, label: T) -> Self {
        Self {
            accessibility_label: Some(label.into()),
            ..self
        }
    }
}
