use super::{
    RichText, RichTextInput,
    composition_objects::{DispatchActionConfiguration, Text},
};

impl RichTextInput {
    /// Construct a [`RichTextInputBuilder`].
    pub fn builder() -> RichTextInputBuilder {
        RichTextInputBuilder::default()
    }
}

/// Builder for [`RichTextInput`] object.
#[derive(Debug, Default)]
pub struct RichTextInputBuilder {
    action_id: Option<String>,
    initial_value: Option<RichText>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl RichTextInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// let rich_text = RichTextInput::builder()
    ///     .set_action_id(Some("rich_text_input-action".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "action_id": "rich_text_input-action"
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// let rich_text = RichTextInput::builder()
    ///     .action_id("rich_text_input-action")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "action_id": "rich_text_input-action"
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// # use slack_messaging::blocks::rich_text::elements::{RichTextSection, RichTextElementTypeText};
    /// let rich_text = RichTextInput::builder()
    ///     .set_initial_value(
    ///         Some(RichText::builder()
    ///             .element(
    ///                 RichTextSection::builder()
    ///                     .element(
    ///                         RichTextElementTypeText::builder()
    ///                             .text("Hello")
    ///                             .build()
    ///                     )
    ///                     .build()
    ///             )
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "initial_value": {
    ///         "type": "rich_text",
    ///         "elements": [
    ///             {
    ///                 "type": "rich_text_section",
    ///                 "elements": [
    ///                     {
    ///                         "type": "text",
    ///                         "text": "Hello"
    ///                     }
    ///                 ]
    ///             }
    ///         ]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_value(self, initial_value: Option<RichText>) -> Self {
        Self {
            initial_value,
            ..self
        }
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// # use slack_messaging::blocks::rich_text::elements::{RichTextSection, RichTextElementTypeText};
    /// let rich_text = RichTextInput::builder()
    ///     .initial_value(
    ///         RichText::builder()
    ///             .element(
    ///                 RichTextSection::builder()
    ///                     .element(
    ///                         RichTextElementTypeText::builder()
    ///                             .text("Hello")
    ///                             .build()
    ///                     )
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "initial_value": {
    ///         "type": "rich_text",
    ///         "elements": [
    ///             {
    ///                 "type": "rich_text_section",
    ///                 "elements": [
    ///                     {
    ///                         "type": "text",
    ///                         "text": "Hello"
    ///                     }
    ///                 ]
    ///             }
    ///         ]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_value(self, initial_value: RichText) -> Self {
        self.set_initial_value(Some(initial_value))
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let rich_text = RichTextInput::builder()
    ///     .set_dispatch_action_config(
    ///         Some(DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnCharacterEntered)
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered", "on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
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
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let rich_text = RichTextInput::builder()
    ///     .dispatch_action_config(
    ///         DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnCharacterEntered)
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered", "on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// let rich_text = RichTextInput::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
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
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// let rich_text = RichTextInput::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// # use slack_messaging::composition_objects::Text;
    /// let rich_text = RichTextInput::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Enter text")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter text"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
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
    /// # use slack_messaging::blocks::elements::RichTextInput;
    /// let rich_text = RichTextInput::builder()
    ///     .placeholder("Enter text")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter text"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`RichTextInput`] object.
    pub fn build(self) -> RichTextInput {
        RichTextInput {
            kind: "rich_text_input",
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
    pub fn get_initial_value(&self) -> &Option<RichText> {
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
    pub fn get_placeholder(&self) -> &Option<Text> {
        &self.placeholder
    }
}
