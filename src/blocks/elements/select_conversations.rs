use super::composition_objects::{ConfirmationDialog, ConversationFilter, Text};
use serde::Serialize;

/// [Select menu of conversations element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectConversations;
/// let menu = SelectConversations::builder()
///     .action_id("text1234")
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "conversations_select",
///     "action_id": "text1234",
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
pub struct SelectConversations {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) response_url_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<ConversationFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
