use super::composition_objects::PlainText;
use serde::Serialize;

/// [Feedback buttons element](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element) representation.
///
/// The Builder returns [`FeedbackButtonsError`](crate::blocks::elements::builders::FeedbackButtonsError) or [`FeedbackButtonError`](crate::blocks::elements::builders::FeedbackButtonError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
/// use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let buttons = FeedbackButtons::builder()
///     .action_id("feedback_buttons_1")
///     .positive_button(
///         FeedbackButton::builder()
///             .text(plain_text!("Good")?)
///             .value("positive_feedback")
///             .accessibility_label("Mark this response as good")
///             .build()?
///     )
///     .negative_button(
///         FeedbackButton::builder()
///             .text(plain_text!("Bad")?)
///             .value("negative_feedback")
///             .accessibility_label("Mark this response as bad")
///             .build()?
///     )
///     .build()?;
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
/// let json = serde_json::to_value(buttons).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = FeedbackButton::builder().build();
/// assert!(button.is_err());
///
/// let buttons = FeedbackButtons::builder()
///     .positive_button(
///         FeedbackButton::builder()
///             .text(plain_text!("Good")?)
///             .value("positive_feedback")
///             .accessibility_label("Mark this response as good")
///             .build()?
///     )
///     .build();
/// assert!(buttons.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "feedback_buttons")]
pub struct FeedbackButtons {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    pub(crate) positive_button: Option<FeedbackButton>,

    pub(crate) negative_button: Option<FeedbackButton>,
}

/// Button object to be set to the `positive_buttons` and `negative_buttons`
/// fields of [`FeedbackButtons`] object.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct FeedbackButton {
    pub(crate) text: Option<PlainText>,

    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accessibility_label: Option<String>,
}
