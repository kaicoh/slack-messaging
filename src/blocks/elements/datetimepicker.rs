use super::ConfirmationDialog;
use serde::Serialize;

/// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::DatetimePicker;
/// use serde_json::json;
///
/// let datetimepicker = DatetimePicker::new()
///     .set_action_id("datetime_input")
///     .set_initial_date_time(1628633820);
///
/// let expected = json!({
///     "type": "datetimepicker",
///     "action_id": "datetime_input",
///     "initial_date_time": 1628633820
/// });
///
/// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
///
/// assert_eq!(datetimepicker_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct DatetimePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl Default for DatetimePicker {
    fn default() -> Self {
        Self {
            kind: "datetimepicker",
            action_id: "".into(),
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}

impl DatetimePicker {
    /// Constructs a Date picker element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::new();
    ///
    /// let expected = json!({
    ///     "type": "datetimepicker",
    ///     "action_id": ""
    /// });
    ///
    /// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(datetimepicker_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::new()
    ///     .set_action_id("datetimepicker_input");
    ///
    /// let expected = json!({
    ///     "type": "datetimepicker",
    ///     "action_id": "datetimepicker_input"
    /// });
    ///
    /// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(datetimepicker_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_date_time field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::new().set_initial_date_time(1628633820);
    ///
    /// let expected = json!({
    ///     "type": "datetimepicker",
    ///     "action_id": "",
    ///     "initial_date_time": 1628633820
    /// });
    ///
    /// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(datetimepicker_json, expected);
    /// ```
    pub fn set_initial_date_time<T: Into<i64>>(self, datetime: T) -> Self {
        Self {
            initial_date_time: Some(datetime.into()),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{ConfirmationDialog, DatetimePicker};
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "datetimepicker",
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
    /// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(datetimepicker_json, expected);
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
    /// use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "datetimepicker",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let datetimepicker_json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(datetimepicker_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }
}
