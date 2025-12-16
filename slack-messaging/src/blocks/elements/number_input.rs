use crate::composition_objects::{DispatchActionConfiguration, Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Number input element](https://docs.slack.dev/reference/block-kit/block-elements/number-input-element)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/number-input-element).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | is_decimal_allowed | bool | Yes | N/A |
/// | action_id | String | No | Max length 255 characters |
/// | initial_value | String | No | N/A |
/// | min_value | String | No | N/A |
/// | max_value | String | No | N/A |
/// | dispatch_action_config | [DispatchActionConfiguration] | No | N/A |
/// | focus_on_load | bool | No | N/A |
/// | placeholder | [Text<Plain>] | No | Max length 150 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::NumberInput;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let num = NumberInput::builder()
///     .action_id("input_number")
///     .is_decimal_allowed(true)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "number_input",
///     "action_id": "input_number",
///     "is_decimal_allowed": true
/// });
///
/// let json = serde_json::to_value(num).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let num = NumberInput::builder()
///     .action_id("input_number")
///     .build();
///
/// assert!(num.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "number_input")]
pub struct NumberInput {
    #[builder(validate("required"))]
    pub(crate) is_decimal_allowed: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_value: Option<String>,

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
        let expected = NumberInput {
            is_decimal_allowed: Some(true),
            action_id: Some("number_input_0".into()),
            initial_value: Some("5.0".into()),
            min_value: Some("0.0".into()),
            max_value: Some("10.0".into()),
            dispatch_action_config: Some(dispatch_action_config()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Input Number")),
        };

        let val = NumberInput::builder()
            .set_is_decimal_allowed(Some(true))
            .set_action_id(Some("number_input_0"))
            .set_initial_value(Some("5.0"))
            .set_min_value(Some("0.0"))
            .set_max_value(Some("10.0"))
            .set_dispatch_action_config(Some(dispatch_action_config()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Input Number")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = NumberInput::builder()
            .is_decimal_allowed(true)
            .action_id("number_input_0")
            .initial_value("5.0")
            .min_value("0.0")
            .max_value("10.0")
            .dispatch_action_config(dispatch_action_config())
            .focus_on_load(true)
            .placeholder(plain_text("Input Number"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_is_decimal_allowed_field() {
        let err = NumberInput::builder().build().unwrap_err();
        assert_eq!(err.object(), "NumberInput");

        let errors = err.field("is_decimal_allowed");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = NumberInput::builder()
            .is_decimal_allowed(true)
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "NumberInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = NumberInput::builder()
            .is_decimal_allowed(true)
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "NumberInput");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }
}
