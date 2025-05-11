use super::composition_objects::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

/// [Multi-select menu External data source element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectExternals;
/// let menu = MultiSelectExternals::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder("Select items")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
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
pub struct MultiSelectExternals {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_query_length: Option<i64>,

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
