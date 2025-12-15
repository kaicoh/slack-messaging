use crate::composition_objects::{ConfirmationDialog, PlainText};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::DatePicker;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let datepicker = DatePicker::builder()
///     .action_id("datepicker-123")
///     .initial_date("1990-04-28")
///     .placeholder(plain_text!("Select a date")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "datepicker",
///     "action_id": "datepicker-123",
///     "initial_date": "1990-04-28",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select a date"
///     }
/// });
///
/// let json = serde_json::to_value(datepicker).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let datepicker = DatePicker::builder()
///     .action_id("datepicker-123")
///     .initial_date("1990-04-31")
///     .placeholder(plain_text!("Select a date")?)
///     .build();
///
/// assert!(datepicker.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "datepicker")]
pub struct DatePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::date_format"))]
    pub(crate) initial_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

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
        let expected = DatePicker {
            action_id: Some("datepicker_0".into()),
            initial_date: Some("2025-12-10".into()),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select a date")),
        };

        let val = DatePicker::builder()
            .set_action_id(Some("datepicker_0"))
            .set_initial_date(Some("2025-12-10"))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select a date")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DatePicker::builder()
            .action_id("datepicker_0")
            .initial_date("2025-12-10")
            .confirm(confirm())
            .focus_on_load(true)
            .placeholder(plain_text("Select a date"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = DatePicker::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DatePicker");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_initial_date_matches_date_format() {
        let err = DatePicker::builder()
            .initial_date("foobar")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DatePicker");

        let errors = err.field("initial_date");
        assert!(errors.includes(ValidationErrorKind::InvalidFormat("YYYY-MM-DD")));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = DatePicker::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DatePicker");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }
}
