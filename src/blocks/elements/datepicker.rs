use super::composition_objects::{ConfirmationDialog, PlainText};
use serde::Serialize;

/// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::DatePicker;
/// let datepicker = DatePicker::builder()
///     .action_id("datepicker-123")
///     .initial_date("1990-04-28")
///     .placeholder("Select a date")
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DatePicker {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<PlainText>,
}
