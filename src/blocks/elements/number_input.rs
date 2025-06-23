use super::composition_objects::{DispatchActionConfiguration, PlainText};
use serde::Serialize;

/// [Number input element](https://docs.slack.dev/reference/block-kit/block-elements/number-input-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::NumberInput;
/// let num = NumberInput::builder()
///     .action_id("input_number")
///     .is_decimal_allowed(true)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "number_input",
///     "action_id": "input_number",
///     "is_decimal_allowed": true
/// });
///
/// let json = serde_json::to_value(num).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct NumberInput {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) is_decimal_allowed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<PlainText>,
}
