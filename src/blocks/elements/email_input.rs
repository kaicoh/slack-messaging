use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [Email input element](https://api.slack.com/reference/block-kit/block-elements#email)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::EmailInput;
/// use serde_json::json;
///
/// let email = EmailInput::new()
///     .set_action_id("input_email")
///     .placeholder("Enter an email");
///
/// let expected = json!({
///     "type": "email_text_input",
///     "action_id": "input_email",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter an email",
///         "emoji": true
///     }
/// });
///
/// let email_json = serde_json::to_value(email).unwrap();
///
/// assert_eq!(email_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct EmailInput {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for EmailInput {
    fn default() -> Self {
        Self {
            kind: "email_text_input",
            action_id: "".into(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl EmailInput {
    /// Constructs a Email Input element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::EmailInput;
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new();
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": ""
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::EmailInput;
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new().set_action_id("input_email");
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "input_email"
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_value field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::EmailInput;
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new().set_initial_value("tanaka@gmail.com");
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "",
    ///     "initial_value": "tanaka@gmail.com"
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    /// Sets dispatch_action_config field with DispatchActionConfiguration.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{EmailInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new()
    ///     .set_dispatch_action_config(
    ///         DispatchActionConfiguration::new()
    ///             .push_trigger_action(TriggerAction::OnEnterPressed)
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": [
    ///             "on_enter_pressed"
    ///         ]
    ///     }
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn set_dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        Self {
            dispatch_action_config: Some(config),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::EmailInput;
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }

    /// Sets placeholder field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{EmailInput, Text};
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new().set_placeholder(Text::plain("Enter your email."));
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter your email.",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    /// Sets placeholder field from string. This is a shorthand for `set_placeholder` method.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::EmailInput;
    /// use serde_json::json;
    ///
    /// let email = EmailInput::new().placeholder("Enter your email.");
    ///
    /// let expected = json!({
    ///     "type": "email_text_input",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter your email.",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let email_json = serde_json::to_value(email).unwrap();
    ///
    /// assert_eq!(email_json, expected);
    /// ```
    pub fn placeholder(self, placeholder: Text) -> Self {
        self.set_placeholder(placeholder)
    }
}
