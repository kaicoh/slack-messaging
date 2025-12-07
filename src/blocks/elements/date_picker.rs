use super::composition_objects::{ConfirmationDialog, PlainText};
use serde::Serialize;

/// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
/// representation.
///
/// The Builder returns [`DatePickerError`](crate::blocks::elements::builders::DatePickerError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
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
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "datepicker")]
pub struct DatePicker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) placeholder: Option<PlainText>,
}
