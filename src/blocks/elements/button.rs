use super::composition_objects::{ConfirmationDialog, PlainText};
use serde::Serialize;

/// [Button element](https://docs.slack.dev/reference/block-kit/block-elements/button-element)
/// representation.
///
/// The Builder returns [`ButtonError`](crate::blocks::elements::builders::ButtonError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
/// use slack_messaging::blocks::elements::Button;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = Button::builder()
///     .text(plain_text!("Click Me")?)
///     .value("click_me_123")
///     .action_id("button-0")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "button",
///     "text": {
///         "type": "plain_text",
///         "text": "Click Me"
///     },
///     "value": "click_me_123",
///     "action_id": "button-0"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let text = Button::builder()
///     .value("click_me_123")
///     .action_id("button-0")
///     .build();
///
/// assert!(text.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "button")]
pub struct Button {
    pub(crate) text: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accessibility_label: Option<String>,
}
