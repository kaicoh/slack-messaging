use super::composition_objects::ConfirmationDialog;
use serde::Serialize;

/// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::DatetimePicker;
/// let datetimepicker = DatetimePicker::builder()
///     .action_id("datetime_input")
///     .initial_date_time(1628633820)
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DatetimePicker {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,
}
