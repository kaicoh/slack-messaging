use crate::composition_objects::ConfirmationDialog;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Datetime picker element](https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::DatetimePicker;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let datetimepicker = DatetimePicker::builder()
///     .action_id("datetime_input")
///     .initial_date_time(1628633820)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "datetimepicker",
///     "action_id": "datetime_input",
///     "initial_date_time": 1628633820
/// });
///
/// let json = serde_json::to_value(datetimepicker).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let datetimepicker = DatetimePicker::builder()
///     .action_id("datetime_input")
///     .initial_date_time(1000)
///     .build();
///
/// assert!(datetimepicker.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "datetimepicker")]
pub struct DatetimePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::ten_digits"))]
    pub(crate) initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DatetimePicker {
            action_id: Some("datetimepicker_0".into()),
            initial_date_time: Some(1628633820),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
        };

        let val = DatetimePicker::builder()
            .set_action_id(Some("datetimepicker_0"))
            .set_initial_date_time(Some(1628633820))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DatetimePicker::builder()
            .action_id("datetimepicker_0")
            .initial_date_time(1628633820)
            .confirm(confirm())
            .focus_on_load(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = DatetimePicker::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DatetimePicker");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_initial_date_time_matches_10_digits_format() {
        let err = DatetimePicker::builder()
            .initial_date_time(100000)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DatetimePicker");

        let errors = err.field("initial_date_time");
        assert!(errors.includes(ValidationErrorKind::InvalidFormat("10 digits")));
    }
}
