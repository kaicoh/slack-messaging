use super::composition_objects::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Checkboxes](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::Checkboxes;
/// # use slack_messaging::composition_objects::Opt;
/// let checkboxes = Checkboxes::builder()
///     .action_id("group-0")
///     .option(
///         Opt::builder()
///             .text("option-0")
///             .value("value-0")
///             .build()
///     )
///     .option(
///         Opt::builder()
///             .text("option-1")
///             .value("value-1")
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "checkboxes",
///     "action_id": "group-0",
///     "options": [
///         {
///             "value": "value-0",
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0"
///             }
///         },
///         {
///             "value": "value-1",
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(checkboxes).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Checkboxes {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,
}
