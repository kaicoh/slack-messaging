use super::composition_objects::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Multi-select menu Public channels element](https://api.slack.com/reference/block-kit/block-elements#channel_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
/// let menu = MultiSelectPublicChannels::builder()
///     .action_id("text1234")
///     .placeholder("Select channels")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_channels_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select channels"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectPublicChannels {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_channels: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
