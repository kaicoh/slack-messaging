use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [Plain-text input element](https://api.slack.com/reference/block-kit/block-elements#input)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::PlainTextInput;
/// let plain = PlainTextInput::builder()
///     .action_id("plain_input")
///     .multiline(true)
///     .placeholder("Enter some plain text")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "plain_text_input",
///     "action_id": "plain_input",
///     "multiline": true,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter some plain text"
///     }
/// });
///
/// let json = serde_json::to_value(plain).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PlainTextInput {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

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

impl PlainTextInput {
    /// Construct a [`PlainTextInputBuilder`].
    pub fn builder() -> PlainTextInputBuilder {
        PlainTextInputBuilder::default()
    }
}

/// Builder for [`PlainTextInput`] object.
#[derive(Debug, Default)]
pub struct PlainTextInputBuilder {
    action_id: Option<String>,
    initial_value: Option<String>,
    multiline: Option<bool>,
    min_length: Option<i64>,
    max_length: Option<i64>,
    dispatch_action_config: Option<DispatchActionConfiguration>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl PlainTextInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_action_id(Some("plain_input".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "plain_input"
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .action_id("plain_input")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "action_id": "plain_input"
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_initial_value(Some("some value".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "initial_value": "some value"
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
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
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .initial_value("some value")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "initial_value": "some value"
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_value(self, initial_value: impl Into<String>) -> Self {
        self.set_initial_value(Some(initial_value.into()))
    }

    /// Set multiline field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_multiline(Some(false))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "multiline": false
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_multiline(self, multiline: Option<bool>) -> Self {
        Self { multiline, ..self }
    }

    /// Set multiline field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .multiline(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "multiline": false
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn multiline(self, multiline: bool) -> Self {
        self.set_multiline(Some(multiline))
    }

    /// Set min_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_min_length(Some(10))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "min_length": 10
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_min_length(self, min_length: Option<i64>) -> Self {
        Self { min_length, ..self }
    }

    /// Set min_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .min_length(10)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "min_length": 10
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn min_length(self, min_length: impl Into<i64>) -> Self {
        self.set_min_length(Some(min_length.into()))
    }

    /// Set max_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_max_length(Some(200))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "max_length": 200
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_length(self, max_length: Option<i64>) -> Self {
        Self { max_length, ..self }
    }

    /// Set max_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .max_length(200)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "max_length": 200
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_length(self, max_length: impl Into<i64>) -> Self {
        self.set_max_length(Some(max_length.into()))
    }

    /// Set dispatch_action_config field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{PlainTextInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// let plain = PlainTextInput::builder()
    ///     .set_dispatch_action_config(
    ///         Some(DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
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
    /// # use slack_messaging::blocks::elements::{PlainTextInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// let plain = PlainTextInput::builder()
    ///     .dispatch_action_config(
    ///         DispatchActionConfiguration::builder()
    ///             .trigger_action(TriggerAction::OnEnterPressed)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_enter_pressed"]
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
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
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
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
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .set_placeholder(Some(plain_text!("Enter some text.")))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter some text."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
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
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let plain = PlainTextInput::builder()
    ///     .placeholder("Enter some text.")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text_input",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter some text."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(plain).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`PlainTextInput`] object.
    pub fn build(self) -> PlainTextInput {
        PlainTextInput {
            kind: "plain_text_input",
            action_id: self.action_id,
            initial_value: self.initial_value,
            multiline: self.multiline,
            min_length: self.min_length,
            max_length: self.max_length,
            dispatch_action_config: self.dispatch_action_config,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }
}
