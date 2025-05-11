use super::composition_objects::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

/// [Select menu of external data source element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectExternals;
/// let menu = SelectExternals::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
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
pub struct SelectExternals {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
