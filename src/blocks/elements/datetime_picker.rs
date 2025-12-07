use super::composition_objects::ConfirmationDialog;
use serde::Serialize;

/// [Datetime picker element](https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element)
/// representation.
///
/// The Builder returns [`DatetimePickerError`](crate::blocks::elements::builders::DatetimePickerError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
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
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "datetimepicker")]
pub struct DatetimePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,
}
