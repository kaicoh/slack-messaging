use super::composition_objects::{DispatchActionConfiguration, Text};
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
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) multiline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
