use super::{
    composition_objects::{ConfirmationDialog, Text},
    DatePicker,
};

impl DatePicker {
    /// Construct a [`DatePickerBuilder`].
    pub fn builder() -> DatePickerBuilder {
        DatePickerBuilder::default()
    }
}

#[derive(Debug, Default)]
pub struct DatePickerBuilder {
    action_id: Option<String>,
    initial_date: Option<String>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl DatePickerBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .set_action_id(Some("datepicker-123".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "action_id": "datepicker-123"
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .action_id("datepicker-123")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "action_id": "datepicker-123"
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_date field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .set_initial_date(Some("1990-04-28".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "initial_date": "1990-04-28"
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_date(self, initial_date: Option<String>) -> Self {
        Self {
            initial_date,
            ..self
        }
    }

    /// Set initial_date field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .initial_date("1990-04-28")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "initial_date": "1990-04-28"
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_date(self, initial_date: impl Into<String>) -> Self {
        self.set_initial_date(Some(initial_date.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let datepicker = DatePicker::builder()
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
    ///     "type": "datepicker",
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
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let datepicker = DatePicker::builder()
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
    ///     "type": "datepicker",
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
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
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
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// # use slack_messaging::composition_objects::Text;
    /// let datepicker = DatePicker::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select a date")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a date"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
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
    /// # use slack_messaging::blocks::elements::DatePicker;
    /// let datepicker = DatePicker::builder()
    ///     .placeholder("Select a date")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "datepicker",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select a date"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(datepicker).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`DatePicker`] object.
    pub fn build(self) -> DatePicker {
        DatePicker {
            kind: "datepicker",
            action_id: self.action_id,
            initial_date: self.initial_date,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_date value.
    pub fn get_initial_date(&self) -> &Option<String> {
        &self.initial_date
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
    pub fn get_placeholder(&self) -> &Option<Text> {
        &self.placeholder
    }
}
