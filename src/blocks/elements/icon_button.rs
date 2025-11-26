use super::composition_objects::{ConfirmationDialog, PlainText};
use serde::Serialize;

/// [Icon button element](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::IconButton;
/// let button = IconButton::builder()
///     .icon("trash")
///     .text("Delete")
///     .action_id("delete_button")
///     .value("delete_item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "icon_button",
///     "icon": "trash",
///     "text": {
///       "type": "plain_text",
///       "text": "Delete"
///     },
///     "action_id": "delete_button",
///     "value": "delete_item"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct IconButton {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) icon: String,

    pub(super) text: PlainText,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) accessibility_label: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) visible_to_user_ids: Vec<String>,
}
