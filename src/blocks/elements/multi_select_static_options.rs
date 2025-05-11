use super::composition_objects::{ConfirmationDialog, Opt, OptGroup, Text};
use serde::Serialize;

/// [Multi-select menu Static options element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// let menu = MultiSelectStaticOptions::builder()
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
///     .placeholder("Select items")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_static_select",
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
///         "text": "Select items"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectStaticOptions {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) option_groups: Vec<OptGroup>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
