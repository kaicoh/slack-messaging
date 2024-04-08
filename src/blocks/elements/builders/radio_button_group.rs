use super::{
    composition_objects::{ConfirmationDialog, Opt},
    RadioButtonGroup,
};

impl RadioButtonGroup {
    /// Construct a [`RadioButtonGroupBuilder`].
    pub fn builder() -> RadioButtonGroupBuilder {
        RadioButtonGroupBuilder::default()
    }
}

/// Builder for [`RadioButtonGroup`] object.
#[derive(Debug, Default)]
pub struct RadioButtonGroupBuilder {
    action_id: Option<String>,
    options: Vec<Opt>,
    initial_option: Option<Opt>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
}

impl RadioButtonGroupBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// let radio = RadioButtonGroup::builder()
    ///     .set_action_id(Some("radio_button_group".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "radio_button_group",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// let radio = RadioButtonGroup::builder()
    ///     .action_id("radio_button_group")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "radio_button_group",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::Opt;
    /// let radio = RadioButtonGroup::builder()
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("Radio 1")
    ///                 .value("A1")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text("Radio 2")
    ///                 .value("A2")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [
    ///         {
    ///             "value": "A1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 1"
    ///             }
    ///         },
    ///         {
    ///             "value": "A2",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 2"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Add Opt object to options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::Opt;
    /// let radio = RadioButtonGroup::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text("Radio 1")
    ///             .value("A1")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [
    ///         {
    ///             "value": "A1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 1"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::Opt;
    /// let radio = RadioButtonGroup::builder()
    ///     .set_initial_option(
    ///         Some(Opt::builder()
    ///             .text("Radio 1")
    ///             .value("A1")
    ///             .build()),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [],
    ///     "initial_option": {
    ///        "value": "A1",
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "Radio 1"
    ///        }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_option(self, initial_option: Option<Opt>) -> Self {
        Self {
            initial_option,
            ..self
        }
    }

    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::Opt;
    /// let radio = RadioButtonGroup::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text("Radio 1")
    ///             .value("A1")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [],
    ///     "initial_option": {
    ///        "value": "A1",
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "Radio 1"
    ///        }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_option(self, initial_option: Opt) -> Self {
        self.set_initial_option(Some(initial_option))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let radio = RadioButtonGroup::builder()
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
    ///     "type": "radio_buttons",
    ///     "options": [],
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
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let radio = RadioButtonGroup::builder()
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
    ///     "type": "radio_buttons",
    ///     "options": [],
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
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// let radio = RadioButtonGroup::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
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
    /// # use slack_messaging::blocks::elements::RadioButtonGroup;
    /// let radio = RadioButtonGroup::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "radio_buttons",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Build a [`RadioButtonGroup`] object.
    pub fn build(self) -> RadioButtonGroup {
        RadioButtonGroup {
            kind: "radio_buttons",
            action_id: self.action_id,
            options: self.options,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get options value.
    pub fn get_options(&self) -> &[Opt] {
        &self.options
    }

    /// Get initial_option value.
    pub fn get_initial_option(&self) -> &Option<Opt> {
        &self.initial_option
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get focus_on_load value.
    pub fn get_focus_on_load(&self) -> &Option<bool> {
        &self.focus_on_load
    }
}
