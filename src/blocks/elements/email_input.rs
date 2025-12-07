use super::composition_objects::{DispatchActionConfiguration, PlainText};
use serde::Serialize;

/// [Email input element](https://docs.slack.dev/reference/block-kit/block-elements/email-input-element)
/// representation.
///
/// The Builder returns [`EmailInputError`](crate::blocks::elements::builders::EmailInputError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::blocks::elements::EmailInput;
/// use slack_messaging::composition_objects::PlainText;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let input = EmailInput::builder()
///     .action_id("input_email")
///     .placeholder(
///         PlainText::builder()
///             .text("Enter an email")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "email_text_input",
///     "action_id": "input_email",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter an email"
///     }
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let input = EmailInput::builder()
///     .action_id("input_email")
///     .placeholder(
///         PlainText::builder()
///             .text("verrrrrrry long text".repeat(10))
///             .build()?
///     )
///     .build();
///
/// assert!(input.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "email_text_input")]
pub struct EmailInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) placeholder: Option<PlainText>,
}
