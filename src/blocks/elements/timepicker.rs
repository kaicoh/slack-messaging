use super::composition_objects::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::TimePicker;
/// let timepicker = TimePicker::builder()
///     .action_id("timepicker123")
///     .initial_time("11:30")
///     .timezone("Asia/Tokyo")
///     .placeholder("Select a time")
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct TimePicker {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) timezone: Option<String>,
}
