use super::ConfirmationDialog;
use serde::Serialize;

/// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::DatetimePicker;
/// let datetimepicker = DatetimePicker::builder()
///     .action_id("datetime_input")
///     .initial_date_time(1628633820)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "datetimepicker",
///     "action_id": "datetime_input",
///     "initial_date_time": 1628633820
/// });
///
/// let json = serde_json::to_value(datetimepicker).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DatetimePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl DatetimePicker {
    /// Construct a [`DatetimePickerBuilder`].
    pub fn builder() -> DatetimePickerBuilder {
        DatetimePickerBuilder::default()
    }
}

/// Builder for [`DatetimePicker`] object.
#[derive(Debug, Default)]
pub struct DatetimePickerBuilder {
    action_id: Option<String>,
    initial_date_time: Option<i64>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
}

impl DatetimePickerBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// let datetimepicker = DatetimePicker::builder()
    ///     .set_action_id(Some("datetimepicker_input".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "action_id": "datetimepicker_input"
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// let datetimepicker = DatetimePicker::builder()
    ///     .action_id("datetimepicker_input")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "action_id": "datetimepicker_input"
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_date_time field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::builder()
    ///     .set_initial_date_time(Some(1628633820))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "initial_date_time": 1628633820
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_date_time(self, datetime: Option<i64>) -> Self {
        Self {
            initial_date_time: datetime,
            ..self
        }
    }

    /// Set initial_date_time field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// use serde_json::json;
    ///
    /// let datetimepicker = DatetimePicker::builder()
    ///     .initial_date_time(1628633820)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "initial_date_time": 1628633820
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_date_time(self, datetime: impl Into<i64>) -> Self {
        self.set_initial_date_time(Some(datetime.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{ConfirmationDialog, DatetimePicker};
    /// let datetimepicker = DatetimePicker::builder()
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
    ///     "type": "datetimepicker",
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
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{ConfirmationDialog, DatetimePicker};
    /// let datetimepicker = DatetimePicker::builder()
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
    ///     "type": "datetimepicker",
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
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// let datetimepicker = DatetimePicker::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
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
    /// # use slack_messaging::blocks::elements::DatetimePicker;
    /// let datetimepicker = DatetimePicker::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datetimepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(datetimepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Build a [`DatetimePicker`] object.
    pub fn build(self) -> DatetimePicker {
        DatetimePicker {
            kind: "datetimepicker",
            action_id: self.action_id,
            initial_date_time: self.initial_date_time,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
        }
    }
}
