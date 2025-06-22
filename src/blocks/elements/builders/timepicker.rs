use super::{
    TimePicker,
    composition_objects::{ConfirmationDialog, PlainText},
};

impl TimePicker {
    /// Construct a [`TimePickerBuilder`].
    pub fn builder() -> TimePickerBuilder {
        TimePickerBuilder::default()
    }
}

/// Builder for [`TimePicker`] object.
#[derive(Debug, Default)]
pub struct TimePickerBuilder {
    action_id: Option<String>,
    initial_time: Option<String>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
    timezone: Option<String>,
}

impl TimePickerBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .set_action_id(Some("timepicker123".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "action_id": "timepicker123"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .action_id("timepicker123")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "action_id": "timepicker123"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_time field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .set_initial_time(Some("11:00".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "initial_time": "11:00"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_time(self, initial_time: Option<String>) -> Self {
        Self {
            initial_time,
            ..self
        }
    }

    /// Set initial_time field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .initial_time("11:00")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "initial_time": "11:00"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_time(self, initial_time: impl Into<String>) -> Self {
        self.set_initial_time(Some(initial_time.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let timepicker = TimePicker::builder()
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
    ///     "type": "timepicker",
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
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let timepicker = TimePicker::builder()
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
    ///     "type": "timepicker",
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
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
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
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let timepicker = TimePicker::builder()
    ///     .set_placeholder(
    ///         Some(PlainText::builder()
    ///             .text("Select a time")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a time"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Option<PlainText>) -> Self {
        Self {
            placeholder,
            ..self
        }
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .placeholder("Select a time")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a time"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = PlainText::builder().text(placeholder).build();
        self.set_placeholder(Some(text))
    }

    /// Set timezone field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .set_timezone(Some("Asia/Tokyo".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "timezone": "Asia/Tokyo"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_timezone(self, timezone: Option<String>) -> Self {
        Self { timezone, ..self }
    }

    /// Set timezone field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::TimePicker;
    /// let timepicker = TimePicker::builder()
    ///     .timezone("Asia/Tokyo")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "timepicker",
    ///     "timezone": "Asia/Tokyo"
    /// });
    ///
    /// let json = serde_json::to_value(timepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn timezone(self, timezone: impl Into<String>) -> Self {
        self.set_timezone(Some(timezone.into()))
    }

    /// Build a [`TimePicker`] object.
    pub fn build(self) -> TimePicker {
        TimePicker {
            kind: "timepicker",
            action_id: self.action_id,
            initial_time: self.initial_time,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            timezone: self.timezone,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_time value.
    pub fn get_initial_time(&self) -> &Option<String> {
        &self.initial_time
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get focus_on_load value.
    pub fn get_focus_on_load(&self) -> &Option<bool> {
        &self.focus_on_load
    }

    /// Get action_id value.
    pub fn get_placeholder(&self) -> &Option<PlainText> {
        &self.placeholder
    }

    /// Get timezone value.
    pub fn get_timezone(&self) -> &Option<String> {
        &self.timezone
    }
}
