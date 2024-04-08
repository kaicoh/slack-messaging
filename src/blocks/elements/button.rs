use super::composition_objects::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::Button;
/// let button = Button::builder()
///     .text("Click Me")
///     .value("click_me_123")
///     .action_id("button-0")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "button",
///     "text": {
///         "type": "plain_text",
///         "text": "Click Me"
///     },
///     "value": "click_me_123",
///     "action_id": "button-0"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) text: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) accessibility_label: Option<String>,
}
