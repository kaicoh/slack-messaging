use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [URL input element](https://api.slack.com/reference/block-kit/block-elements#url)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::UrlInput;
/// use serde_json::json;
///
/// let url = UrlInput::new()
///     .set_action_id("url_input_action")
///     .placeholder("Enter url");
///
/// let expected = json!({
///     "type": "url_text_input",
///     "action_id": "url_input_action",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter url",
///         "emoji": true
///     }
/// });
///
/// let url_json = serde_json::to_value(url).unwrap();
///
/// assert_eq!(url_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct UrlInput {
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

impl Default for UrlInput {
    fn default() -> Self {
        Self {
            kind: "url_text_input",
            action_id: "".into(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl UrlInput {
    /// Constructs a URL input element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::UrlInput;
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new();
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": ""
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::UrlInput;
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new().set_action_id("url_input_action");
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "url_input_action"
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_value field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::UrlInput;
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new().set_initial_value("https://google.com");
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "",
    ///     "initial_value": "https://google.com"
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
    /// ```
    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    /// Sets dispatch_action_config field with DispatchActionConfiguration object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{UrlInput, DispatchActionConfiguration,
    /// TriggerAction};
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new()
    ///     .set_dispatch_action_config(
    ///         DispatchActionConfiguration::new()
    ///             .push_trigger_action(TriggerAction::OnEnterPressed)
    ///             .push_trigger_action(TriggerAction::OnCharacterEntered)
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "",
    ///     "dispatch_action_config": {
    ///         "trigger_actions_on": ["on_enter_pressed", "on_character_entered"]
    ///     }
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
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
    /// use slack_messaging::blocks::elements::UrlInput;
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
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
    /// use slack_messaging::blocks::elements::{UrlInput, Text};
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new()
    ///     .set_placeholder(Text::plain("Enter url"));
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter url",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
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
    /// use slack_messaging::blocks::elements::UrlInput;
    /// use serde_json::json;
    ///
    /// let url = UrlInput::new().placeholder("Enter url");
    ///
    /// let expected = json!({
    ///     "type": "url_text_input",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Enter url",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let url_json = serde_json::to_value(url).unwrap();
    ///
    /// assert_eq!(url_json, expected);
    /// ```
    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }
}
