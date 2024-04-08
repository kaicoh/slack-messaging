use super::{
    composition_objects::{DispatchActionConfiguration, Text},
    NumberInput,
};

impl NumberInput {
    /// Construct a [`NumberInputBuilder`].
    pub fn builder() -> NumberInputBuilder {
        NumberInputBuilder::default()
    }
}

/// Builder for [`NumberInput`] object.
#[derive(Debug, Default)]
pub struct NumberInputBuilder {
    is_decimal_allowed: Option<bool>,
    action_id: Option<String>,
    initial_value: Option<String>,
    min_value: Option<String>,
    max_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl NumberInputBuilder {
    /// Set is_decimal_allowed field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_is_decimal_allowed(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": true
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_is_decimal_allowed(self, is_decimal_allowed: Option<bool>) -> Self {
        Self {
            is_decimal_allowed,
            ..self
        }
    }

    /// Set is_decimal_allowed field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .is_decimal_allowed(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": true
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    #[allow(clippy::wrong_self_convention)]
    pub fn is_decimal_allowed(self, is_decimal_allowed: bool) -> Self {
        self.set_is_decimal_allowed(Some(is_decimal_allowed))
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_action_id(Some("input_number".into()))
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "action_id": "input_number",
    ///     "is_decimal_allowed": false
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .action_id("input_number")
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "action_id": "input_number",
    ///     "is_decimal_allowed": false
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_initial_value(Some("7".into()))
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "initial_value": "7"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_value(self, initial_value: Option<String>) -> Self {
        Self {
            initial_value,
            ..self
        }
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .initial_value("7")
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "initial_value": "7"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_value(self, initial_value: impl Into<String>) -> Self {
        self.set_initial_value(Some(initial_value.into()))
    }

    /// Set min_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_min_value(Some("5".into()))
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "min_value": "5"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_min_value(self, min_value: Option<String>) -> Self {
        Self { min_value, ..self }
    }

    /// Set min_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .min_value("5")
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "min_value": "5"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn min_value(self, min_value: impl Into<String>) -> Self {
        self.set_min_value(Some(min_value.into()))
    }

    /// Set max_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_max_value(Some("10".into()))
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "max_value": "10"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_value(self, max_value: Option<String>) -> Self {
        Self { max_value, ..self }
    }

    /// Set max_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .max_value("10")
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "max_value": "10"
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_value(self, max_value: impl Into<String>) -> Self {
        self.set_max_value(Some(max_value.into()))
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let num = NumberInput::builder()
    ///     .set_dispatch_action_config(
    ///         Some(DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnCharacterEntered)
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build())
    ///     )
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered", "on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_dispatch_action_config(self, config: Option<DispatchActionConfiguration>) -> Self {
        Self {
            dispatch_action_config: config,
            ..self
        }
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let num = NumberInput::builder()
    ///     .dispatch_action_config(
    ///         DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnCharacterEntered)
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build()
    ///     )
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered", "on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
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
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .focus_on_load(true)
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// # use slack_messaging::composition_objects::Text;
    /// let num = NumberInput::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("How old are you?")
    ///             .build())
    ///     )
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "How old are you?"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
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
    /// # use slack_messaging::blocks::elements::NumberInput;
    /// let num = NumberInput::builder()
    ///     .placeholder("How old are you?")
    ///     .is_decimal_allowed(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "How old are you?"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`NumberInput`] object. This method will panic if `is_decimal_allowed` is not set.
    pub fn build(self) -> NumberInput {
        NumberInput {
            kind: "number_input",
            is_decimal_allowed: self
                .is_decimal_allowed
                .expect("is_decimal_allowed must be set to NumberInputBuilder"),
            action_id: self.action_id,
            initial_value: self.initial_value,
            min_value: self.min_value,
            max_value: self.max_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get is_decimal_allowed value.
    pub fn get_is_decimal_allowed(&self) -> &Option<bool> {
        &self.is_decimal_allowed
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_value value.
    pub fn get_initial_value(&self) -> &Option<String> {
        &self.initial_value
    }

    /// Get min_value value.
    pub fn get_min_value(&self) -> &Option<String> {
        &self.min_value
    }

    /// Get max_value value.
    pub fn get_max_value(&self) -> &Option<String> {
        &self.max_value
    }

    /// Get dispatch_action_config value.
    pub fn get_dispatch_action_config(&self) -> &Option<DispatchActionConfiguration> {
        &self.dispatch_action_config
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
