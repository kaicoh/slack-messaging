use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Time picker element](https://api.slack.com/reference/block-kit/block-elements#timepicker)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::TimePicker;
/// use serde_json::json;
///
/// let timepicker = TimePicker::new()
///     .set_action_id("timepicker123")
///     .set_initial_time("11:30")
///     .set_timezone("Asia/Tokyo")
///     .placeholder("Select a time");
///
/// let expected = json!({
///     "type": "timepicker",
///     "action_id": "timepicker123",
///     "initial_time": "11:30",
///     "timezone": "Asia/Tokyo",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select a time",
///         "emoji": true
///     }
/// });
///
/// let timepicker_json = serde_json::to_value(timepicker).unwrap();
///
/// assert_eq!(timepicker_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct TimePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
}

impl Default for TimePicker {
    fn default() -> Self {
        Self {
            kind: "timepicker",
            action_id: "".into(),
            initial_time: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            timezone: None,
        }
    }
}

impl TimePicker {
    /// Constructs a Time picker element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::TimePicker;
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new();
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": ""
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::TimePicker;
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new().set_action_id("timepicker123");
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": "timepicker123"
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_time field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::TimePicker;
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new().set_initial_time("11:00");
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": "",
    ///     "initial_time": "11:00"
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn set_initial_time<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_time: Some(value.into()),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{TimePicker, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
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
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::TimePicker;
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
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
    /// use slack_messaging::blocks::elements::{TimePicker, Text};
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new()
    ///     .set_placeholder(Text::plain("Select a time"));
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a time",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    /// Sets timezone field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::TimePicker;
    /// use serde_json::json;
    ///
    /// let timepicker = TimePicker::new().set_timezone("Asia/Tokyo");
    ///
    /// let expected = json!({
    ///     "type": "timepicker",
    ///     "action_id": "",
    ///     "timezone": "Asia/Tokyo"
    /// });
    ///
    /// let timepicker_json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(timepicker_json, expected);
    /// ```
    pub fn set_timezone<T: Into<String>>(self, value: T) -> Self {
        Self {
            timezone: Some(value.into()),
            ..self
        }
    }
}
