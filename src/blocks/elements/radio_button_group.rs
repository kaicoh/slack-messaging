use super::composition_objects::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

/// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::RadioButtonGroup;
/// # use slack_messaging::composition_objects::{Opt, Text};
/// # use slack_messaging::mrkdwn;
/// let radio = RadioButtonGroup::builder()
///     .action_id("radio_button_group")
///     .option(
///         Opt::<Text>::builder()
///             .text(mrkdwn!("**Radio 1**"))
///             .value("A1")
///             .build()
///     )
///     .option(
///         Opt::<Text>::builder()
///             .text(mrkdwn!("**Radio 2**"))
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
///                 "type": "mrkdwn",
///                 "text": "**Radio 1**"
///             }
///         },
///         {
///             "value": "A2",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "**Radio 2**"
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

    pub(super) options: Vec<Opt<Text>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_option: Option<Opt<Text>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,
}
