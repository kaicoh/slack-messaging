use super::{
    UrlInput,
    composition_objects::{DispatchActionConfiguration, PlainText},
};

impl UrlInput {
    /// Construct a [`UrlInputBuilder`].
    pub fn builder() -> UrlInputBuilder {
        UrlInputBuilder::default()
    }
}

/// Builder for [`UrlInput`] object.
#[derive(Debug, Default)]
pub struct UrlInputBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
}

impl UrlInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .set_action_id(Some("url_input_action".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "action_id": "url_input_action"
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .action_id("url_input_action")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "action_id": "url_input_action"
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .set_initial_value(Some("https://google.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "initial_value": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
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
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .initial_value("https://google.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "initial_value": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_value(self, initial_value: impl Into<String>) -> Self {
        self.set_initial_value(Some(initial_value.into()))
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let url = UrlInput::builder()
    ///     .set_dispatch_action_config(
    ///         Some(DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
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
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let url = UrlInput::builder()
    ///     .dispatch_action_config(
    ///         DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
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
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let url = UrlInput::builder()
    ///     .set_placeholder(
    ///         Some(PlainText::builder()
    ///             .text("Enter url")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter url"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
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
    /// # use slack_messaging::blocks::elements::UrlInput;
    /// let url = UrlInput::builder()
    ///     .placeholder("Enter url")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "url_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter url"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = PlainText::builder().text(placeholder).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`UrlInput`] object.
    pub fn build(self) -> UrlInput {
        UrlInput {
            kind: "url_text_input",
            action_id: self.action_id,
            initial_value: self.initial_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_value value.
    pub fn get_initial_value(&self) -> &Option<String> {
        &self.initial_value
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
    pub fn get_placeholder(&self) -> &Option<PlainText> {
        &self.placeholder
    }
}
