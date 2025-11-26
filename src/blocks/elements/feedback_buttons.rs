use super::composition_objects::PlainText;
use serde::Serialize;

/// [Feedback buttons element](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
/// let button = FeedbackButtons::builder()
///     .set_action_id(Some("feedback_buttons_1".into()))
///     .positive_button(
///         FeedbackButton::builder()
///             .text("Good")
///             .value("positive_feedback")
///             .accessibility_label("Mark this response as good")
///             .build()
///     )
///     .negative_button(
///         FeedbackButton::builder()
///             .text("Bad")
///             .value("negative_feedback")
///             .accessibility_label("Mark this response as bad")
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "feedback_buttons",
///     "action_id": "feedback_buttons_1",
///     "positive_button": {
///         "text": {
///             "type": "plain_text",
///             "text": "Good"
///         },
///         "value": "positive_feedback",
///         "accessibility_label": "Mark this response as good"
///     },
///     "negative_button": {
///         "text": {
///             "type": "plain_text",
///             "text": "Bad"
///         },
///         "value": "negative_feedback",
///         "accessibility_label": "Mark this response as bad"
///     }
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct FeedbackButtons {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    pub(super) positive_button: FeedbackButton,

    pub(super) negative_button: FeedbackButton,
}

/// Button object to be set to the `positive_buttons` and `negative_buttons`
/// fields of [`FeedbackButtons`] object.
#[derive(Debug, Clone, Serialize)]
pub struct FeedbackButton {
    pub(super) text: PlainText,

    pub(super) value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) accessibility_label: Option<String>,
}
