use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [Email input element](https://api.slack.com/reference/block-kit/block-elements#email)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::EmailInput;
/// let email = EmailInput::builder()
///     .action_id("input_email")
///     .placeholder("Enter an email")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "email_text_input",
///     "action_id": "input_email",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter an email"
///     }
/// });
///
/// let json = serde_json::to_value(email).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EmailInput {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl EmailInput {
    /// Construct a [`EmailInputBuilder`].
    pub fn builder() -> EmailInputBuilder {
        EmailInputBuilder::default()
    }
}

/// Builder for [`EmailInput`] object.
#[derive(Debug, Default)]
pub struct EmailInputBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl EmailInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .set_action_id(Some("input_email".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "action_id": "input_email"
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .action_id("input_email")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "action_id": "input_email"
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .set_initial_value(Some("tanaka@gmail.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "initial_value": "tanaka@gmail.com"
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
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
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .initial_value("tanaka@gmail.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "initial_value": "tanaka@gmail.com"
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_value(self, initial_value: impl Into<String>) -> Self {
        self.set_initial_value(Some(initial_value.into()))
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{EmailInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// let email = EmailInput::builder()
    ///     .set_dispatch_action_config(
    ///         Some(DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": [
    ///             "on_enter_pressed"
    ///         ]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
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
    /// # use slack_messaging::blocks::elements::{EmailInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// let email = EmailInput::builder()
    ///     .dispatch_action_config(
    ///         DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": [
    ///             "on_enter_pressed"
    ///         ]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
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
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .set_placeholder(Some(plain_text!("Enter your email.")))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter your email."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
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
    /// use slack_messaging::blocks::elements::EmailInput;
    /// let email = EmailInput::builder()
    ///     .placeholder("Enter your email.")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "email_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter your email."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`EmailInput`] object.
    pub fn build(self) -> EmailInput {
        EmailInput {
            kind: "email_text_input",
            action_id: self.action_id,
            initial_value: self.initial_value,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }
}
