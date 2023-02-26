use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [Number input element](https://api.slack.com/reference/block-kit/block-elements#number)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::NumberInput;
/// use serde_json::json;
///
/// let num = NumberInput::new()
///     .set_action_id("input_number")
///     .decimal_allowed();
///
/// let expected = json!({
///     "type": "number_input",
///     "action_id": "input_number",
///     "is_decimal_allowed": true
/// });
///
/// let num_json = serde_json::to_value(num).unwrap();
///
/// assert_eq!(num_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct NumberInput {
    #[serde(rename = "type")]
    kind: &'static str,

    is_decimal_allowed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for NumberInput {
    fn default() -> Self {
        Self {
            kind: "number_input",
            is_decimal_allowed: false,
            action_id: None,
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl NumberInput {
    /// Constructs a Number input element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new();
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets is_decimal_allowed field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_is_decimal_allowed(true);
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": true
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_is_decimal_allowed(self, is_decimal_allowed: bool) -> Self {
        Self {
            is_decimal_allowed,
            ..self
        }
    }

    /// Sets true to is_decimal_allowed field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().decimal_allowed();
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": true
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn decimal_allowed(self) -> Self {
        self.set_is_decimal_allowed(true)
    }

    /// Sets false to is_decimal_allowed field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().decimal_disallowed();
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn decimal_disallowed(self) -> Self {
        self.set_is_decimal_allowed(false)
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_action_id("input_number");
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "action_id": "input_number",
    ///     "is_decimal_allowed": false
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, value: T) -> Self {
        Self {
            action_id: Some(value.into()),
            ..self
        }
    }

    /// Sets initial_value field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_initial_value("7");
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "initial_value": "7"
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    /// Sets min_value field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_min_value("5");
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "min_value": "5"
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_min_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            min_value: Some(value.into()),
            ..self
        }
    }

    /// Sets max_value field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_max_value("10");
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "max_value": "10"
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_max_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            max_value: Some(value.into()),
            ..self
        }
    }

    /// Sets dispatch_action_config field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{NumberInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new()
    ///     .set_dispatch_action_config(
    ///         DispatchActionConfiguration::new()
    ///             .push_trigger_action(TriggerAction::OnCharacterEntered)
    ///             .push_trigger_action(TriggerAction::OnEnterPressed)
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered", "on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        Self {
            dispatch_action_config: Some(config),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "focus_on_load": true
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }

    /// Sets placeholder field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{NumberInput, Text};
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().set_placeholder(Text::plain("How old are you?"));
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "How old are you?",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    /// Sets placeholder field from string. This is a shorthand for `set_placeholder` method.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::NumberInput;
    /// use serde_json::json;
    ///
    /// let num = NumberInput::new().placeholder("How old are you?");
    ///
    /// let expected = json!({
    ///     "type": "number_input",
    ///     "is_decimal_allowed": false,
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "How old are you?",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let num_json = serde_json::to_value(num).unwrap();
    ///
    /// assert_eq!(num_json, expected);
    /// ```
    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }
}
