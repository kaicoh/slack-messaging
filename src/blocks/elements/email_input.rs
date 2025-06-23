use super::composition_objects::{DispatchActionConfiguration, PlainText};
use serde::Serialize;

/// [Email input element](https://docs.slack.dev/reference/block-kit/block-elements/email-input-element)
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
    pub(super) placeholder: Option<PlainText>,
}
