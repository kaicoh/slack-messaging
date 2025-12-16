use crate::composition_objects::{DispatchActionConfiguration, Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Plain-text input element](https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::PlainTextInput;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let input = PlainTextInput::builder()
///     .action_id("plain_input")
///     .multiline(true)
///     .placeholder(plain_text!("Enter some plain text")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "plain_text_input",
///     "action_id": "plain_input",
///     "multiline": true,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Enter some plain text"
///     }
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let input = PlainTextInput::builder()
///     .action_id("plain_input")
///     .multiline(true)
///     .placeholder(plain_text!("{}", "verrrrrrry long text".repeat(100))?)
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
#[serde(tag = "type", rename = "plain_text_input")]
pub struct PlainTextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) multiline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_0", "integer::max_3000"))]
    pub(crate) min_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1", "integer::max_3000"))]
    pub(crate) max_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<Text<Plain>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = PlainTextInput {
            action_id: Some("plain_text_input_0".into()),
            initial_value: Some("foobar".into()),
            multiline: Some(true),
            min_length: Some(0),
            max_length: Some(3000),
            dispatch_action_config: Some(dispatch_action_config()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Input Text")),
        };

        let val = PlainTextInput::builder()
            .set_action_id(Some("plain_text_input_0"))
            .set_initial_value(Some("foobar"))
            .set_multiline(Some(true))
            .set_min_length(Some(0))
            .set_max_length(Some(3000))
            .set_dispatch_action_config(Some(dispatch_action_config()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Input Text")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = PlainTextInput::builder()
            .action_id("plain_text_input_0")
            .initial_value("foobar")
            .multiline(true)
            .min_length(0)
            .max_length(3000)
            .dispatch_action_config(dispatch_action_config())
            .focus_on_load(true)
            .placeholder(plain_text("Input Text"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = PlainTextInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_min_length_greater_than_0() {
        let err = PlainTextInput::builder()
            .min_length(-1)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("min_length");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(0)));
    }

    #[test]
    fn it_requires_min_length_less_than_3000() {
        let err = PlainTextInput::builder()
            .min_length(3001)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("min_length");
        assert!(errors.includes(ValidationErrorKind::MaxIntegerValue(3000)));
    }

    #[test]
    fn it_requires_max_length_greater_than_1() {
        let err = PlainTextInput::builder().max_length(0).build().unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("max_length");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_max_length_less_than_3000() {
        let err = PlainTextInput::builder()
            .max_length(3001)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("max_length");
        assert!(errors.includes(ValidationErrorKind::MaxIntegerValue(3000)));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = PlainTextInput::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainTextInput");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }
}
