use super::composition_objects::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Multi-select menu User list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectUsers;
/// let menu = MultiSelectUsers::builder()
///     .action_id("text1234")
///     .initial_user("user9999")
///     .placeholder("Select users")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_users_select",
///     "action_id": "text1234",
///     "initial_users": [
///         "user9999"
///     ],
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select users"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectUsers {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_users: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
