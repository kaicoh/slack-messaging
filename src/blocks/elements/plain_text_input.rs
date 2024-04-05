use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [Plain-text input element](https://api.slack.com/reference/block-kit/block-elements#input)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::PlainTextInput;
/// use serde_json::json;
///
/// let plain = PlainTextInput::new()
///     .set_action_id("plain_input")
///     .multiline()
///     .placeholder("Enter some plain text");
///
/// let expected = json!({
///     "type": "plain_text_input",
///     "action_id": "plain_input",
///     "multiline": true,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter some plain text",
///         "emoji": true
///     }
/// });
///
/// let plain_json = serde_json::to_value(plain).unwrap();
///
/// assert_eq!(plain_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PlainTextInput {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    multiline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for PlainTextInput {
    fn default() -> Self {
        Self {
            kind: "plain_text_input",
            action_id: "".into(),
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl PlainTextInput {
    /// Constructs a Plain-text input element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new();
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": ""
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_action_id("plain_input");
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "plain_input"
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, value: T) -> Self {
        Self {
            action_id: value.into(),
            ..self
        }
    }

    /// Sets initial_value field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_initial_value("some value");
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "initial_value": "some value"
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    /// Sets multiline field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_multiline(false);
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "multiline": false
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_multiline(self, value: bool) -> Self {
        Self {
            multiline: Some(value),
            ..self
        }
    }

    /// Sets true to multiline field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().multiline();
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "multiline":true
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn multiline(self) -> Self {
        self.set_multiline(true)
    }

    /// Sets min_length field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_min_length(10);
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "min_length": 10
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_min_length<T: Into<i64>>(self, value: T) -> Self {
        Self {
            min_length: Some(value.into()),
            ..self
        }
    }

    /// Sets max_length field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_max_length(200);
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "max_length": 200
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_max_length<T: Into<i64>>(self, value: T) -> Self {
        Self {
            max_length: Some(value.into()),
            ..self
        }
    }

    /// Sets dispatch_action_config field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{PlainTextInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new()
    ///     .set_dispatch_action_config(
    ///         DispatchActionConfiguration::new()
    ///             .push_trigger_action(TriggerAction::OnCharacterEntered)
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_character_entered"]
    ///     }
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
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
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
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
    /// use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// use serde_json::json;
    ///
    /// let plain = PlainTextInput::new()
    ///     .set_placeholder(Text::plain("Enter some plain text"));
    ///
    /// let expected = json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter some plain text",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let plain_json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(plain_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }
}
