use super::composition_objects::{ConfirmationDialog, Opt, OptGroup, Text};
use serde::Serialize;

/// [Select menu of static options element](https://api.slack.com/reference/block-kit/block-elements#static_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectStaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// let menu = SelectStaticOptions::builder()
///     .action_id("text1234")
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
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "static_select",
///     "action_id": "text1234",
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
///     ],
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct SelectStaticOptions {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) option_groups: Vec<OptGroup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
