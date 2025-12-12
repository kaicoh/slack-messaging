use crate::composition_objects::{DispatchActionConfiguration, PlainText};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Email input element](https://docs.slack.dev/reference/block-kit/block-elements/email-input-element)
/// representation.
///
/// # Example
///
/// ```
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
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "email_text_input")]
pub struct EmailInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<PlainText>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = EmailInput {
            action_id: Some("email_input_0".into()),
            initial_value: Some("example@gmail.com".into()),
            dispatch_action_config: Some(dispatch_action_config()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Enter an email")),
        };

        let val = EmailInput::builder()
            .set_action_id(Some("email_input_0"))
            .set_initial_value(Some("example@gmail.com"))
            .set_dispatch_action_config(Some(dispatch_action_config()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Enter an email")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = EmailInput::builder()
            .action_id("email_input_0")
            .initial_value("example@gmail.com")
            .dispatch_action_config(dispatch_action_config())
            .focus_on_load(true)
            .placeholder(plain_text("Enter an email"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = EmailInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "EmailInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = EmailInput::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "EmailInput");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
