use super::composition_objects::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Overflow menu element](https://api.slack.com/reference/block-kit/block-elements#overflow)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::OverflowMenu;
/// # use slack_messaging::composition_objects::Opt;
/// let overflow = OverflowMenu::builder()
///     .action_id("overflow_0")
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
///     "type": "overflow",
///     "action_id": "overflow_0",
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0"
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(overflow).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct OverflowMenu {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,
}
