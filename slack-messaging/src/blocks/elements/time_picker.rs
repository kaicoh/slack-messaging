use crate::composition_objects::{ConfirmationDialog, PlainText};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::TimePicker;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let timepicker = TimePicker::builder()
///     .action_id("timepicker123")
///     .initial_time("11:30")
///     .timezone("Asia/Tokyo")
///     .placeholder(plain_text!("Select a time")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "timepicker",
///     "action_id": "timepicker123",
///     "initial_time": "11:30",
///     "timezone": "Asia/Tokyo",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select a time"
///     }
/// });
///
/// let json = serde_json::to_value(timepicker).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let timepicker = TimePicker::builder()
///     .action_id("timepicker123")
///     .initial_time("25:30")
///     .build();
///
/// assert!(timepicker.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "timepicker")]
pub struct TimePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::time_format"))]
    pub(crate) initial_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) timezone: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = TimePicker {
            action_id: Some("time_picker_0".into()),
            initial_time: Some("10:30".into()),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select a time")),
            timezone: Some("Asia/Tokyo".into()),
        };

        let val = TimePicker::builder()
            .set_action_id(Some("time_picker_0"))
            .set_initial_time(Some("10:30"))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select a time")))
            .set_timezone(Some("Asia/Tokyo"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = TimePicker::builder()
            .action_id("time_picker_0")
            .initial_time("10:30")
            .confirm(confirm())
            .focus_on_load(true)
            .placeholder(plain_text("Select a time"))
            .timezone("Asia/Tokyo")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = TimePicker::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TimePicker");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_initial_time_matches_time_format() {
        let err = TimePicker::builder()
            .initial_time("foobar")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TimePicker");

        let errors = err.field("initial_time");
        assert!(errors.includes(ValidationErrorKind::InvalidFormat("24-hour format HH:mm")));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = TimePicker::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TimePicker");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
