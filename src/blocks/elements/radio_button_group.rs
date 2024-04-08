use super::composition_objects::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Radio buton group element](https://api.slack.com/reference/block-kit/block-elements#radio)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::RadioButtonGroup;
/// # use slack_messaging::composition_objects::Opt;
/// let radio = RadioButtonGroup::builder()
///     .action_id("radio_button_group")
///     .option(
///         Opt::builder()
///             .text("Radio 1")
///             .value("A1")
///             .build()
///     )
///     .option(
///         Opt::builder()
///             .text("Radio 2")
///             .value("A2")
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "radio_buttons",
///     "action_id": "radio_button_group",
///     "options": [
///         {
///             "value": "A1",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Radio 1"
///             }
///         },
///         {
///             "value": "A2",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Radio 2"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(radio).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RadioButtonGroup {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,
}
