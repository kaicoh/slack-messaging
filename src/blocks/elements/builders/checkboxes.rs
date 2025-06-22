use super::{
    Checkboxes,
    composition_objects::{ConfirmationDialog, Opt, Text},
};

impl Checkboxes {
    /// Construct a [`CheckboxesBuilder`].
    pub fn builder() -> CheckboxesBuilder {
        CheckboxesBuilder::default()
    }
}

/// Builder for [`Checkboxes`] object.
#[derive(Debug, Default)]
pub struct CheckboxesBuilder {
    action_id: Option<String>,
    options: Vec<Opt<Text>>,
    initial_options: Vec<Opt<Text>>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
}

impl CheckboxesBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// let checkboxes = Checkboxes::builder()
    ///     .set_action_id(Some("group-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "action_id": "group-0",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// let checkboxes = Checkboxes::builder()
    ///     .action_id("group-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "action_id": "group-0",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::plain_text;
    /// let checkboxes = Checkboxes::builder()
    ///     .set_options(
    ///         vec![
    ///             Opt::<Text>::builder()
    ///                 .text(plain_text!("option-0"))
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::<Text>::builder()
    ///                 .text(plain_text!("option-1"))
    ///                 .value("value-1")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             }
    ///         },
    ///         {
    ///             "value": "value-1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt<Text>>) -> Self {
        Self { options, ..self }
    }

    /// Add Opt object to options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::mrkdwn;
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::<Text>::builder()
    ///             .text(mrkdwn!("option-0"))
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option(self, option: Opt<Text>) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Set initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::plain_text;
    /// let checkboxes = Checkboxes::builder()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::<Text>::builder()
    ///                 .text(plain_text!("option-0"))
    ///                 .value("value-0")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_options(self, initial_options: Vec<Opt<Text>>) -> Self {
        Self {
            initial_options,
            ..self
        }
    }

    /// Add Opt object to initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::mrkdwn;
    /// let checkboxes = Checkboxes::builder()
    ///     .initial_option(
    ///         Opt::<Text>::builder()
    ///             .text(mrkdwn!("option-0"))
    ///             .value("value-0")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_option(self, initial_option: Opt<Text>) -> Self {
        let Self {
            mut initial_options,
            ..
        } = self;
        initial_options.push(initial_option);
        Self {
            initial_options,
            ..self
        }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let checkboxes = Checkboxes::builder()
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
    ///     "type": "checkboxes",
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
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let checkboxes = Checkboxes::builder()
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
    ///     "type": "checkboxes",
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
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// let checkboxes = Checkboxes::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
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
    /// # use slack_messaging::blocks::elements::Checkboxes;
    /// let checkboxes = Checkboxes::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Build a [`Checkboxes`] object.
    pub fn build(self) -> Checkboxes {
        Checkboxes {
            kind: "checkboxes",
            action_id: self.action_id,
            options: self.options,
            initial_options: self.initial_options,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get options value.
    pub fn get_options(&self) -> &[Opt<Text>] {
        &self.options
    }

    /// Get initial_options value.
    pub fn get_initial_options(&self) -> &[Opt<Text>] {
        &self.initial_options
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
