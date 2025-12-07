use super::composition_objects::{ConfirmationDialog, PlainText};
use serde::Serialize;

/// [Icon button element](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element) representation.
///
/// The Builder returns [`IconButtonError`](crate::blocks::elements::builders::IconButtonError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
/// use slack_messaging::blocks::elements::{IconButton, Icon};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = IconButton::builder()
///     .icon(Icon::Trash)
///     .text(plain_text!("Delete")?)
///     .action_id("delete_button")
///     .value("delete_item")
///     .build()?;
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
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = IconButton::builder()
///     .icon(Icon::Trash)
///     .build();
///
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "icon_button")]
pub struct IconButton {
    pub(crate) icon: Option<Icon>,

    pub(crate) text: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accessibility_label: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) visible_to_user_ids: Vec<String>,
}

/// Icons for [`IconButton`].
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Icon {
    Trash,
}
