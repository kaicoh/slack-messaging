use super::composition_objects::{DispatchActionConfiguration, Text};
use serde::Serialize;

/// [URL input element](https://api.slack.com/reference/block-kit/block-elements#url)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::UrlInput;
/// let url = UrlInput::builder()
///     .action_id("url_input_action")
///     .placeholder("Enter url")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "url_text_input",
///     "action_id": "url_input_action",
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
#[derive(Debug, Clone, Serialize)]
pub struct UrlInput {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
