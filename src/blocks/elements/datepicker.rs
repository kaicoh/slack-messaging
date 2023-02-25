use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::DatePicker;
/// use serde_json::json;
///
/// let datepicker = DatePicker::new()
///     .set_action_id("datepicker-123")
///     .set_initial_date("1990-04-28")
///     .placeholder("Select a date");
///
/// let expected = json!({
///     "type": "datepicker",
///     "action_id": "datepicker-123",
///     "initial_date": "1990-04-28",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select a date",
///         "emoji": true
///     }
/// });
///
/// let datepicker_json = serde_json::to_value(datepicker).unwrap();
///
/// assert_eq!(datepicker_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct DatePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for DatePicker {
    fn default() -> Self {
        Self {
            kind: "datepicker",
            action_id: "".into(),
            initial_date: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl DatePicker {
    /// Constructs a Date picker element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatePicker;
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new();
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": ""
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatePicker;
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new().set_action_id("datepicker-123");
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": "datepicker-123"
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_date field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatePicker;
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new().set_initial_date("1990-04-28");
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": "",
    ///     "initial_date": "1990-04-28"
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
    /// ```
    pub fn set_initial_date<T: Into<String>>(self, initial_date: T) -> Self {
        Self {
            initial_date: Some(initial_date.into()),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{DatePicker, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
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
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
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
    /// use slack_messaging::blocks::elements::DatePicker;
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
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
    /// use slack_messaging::blocks::elements::{DatePicker, Text};
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new()
    ///     .set_placeholder(Text::plain("Select a date"));
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a date",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
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
    /// use slack_messaging::blocks::elements::DatePicker;
    /// use serde_json::json;
    ///
    /// let datepicker = DatePicker::new()
    ///     .placeholder("Select a date");
    ///
    /// let expected = json!({
    ///     "type": "datepicker",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a date",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let datepicker_json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(datepicker_json, expected);
    /// ```
    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }
}
