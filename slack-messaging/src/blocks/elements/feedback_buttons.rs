use crate::blocks::elements::types::FeedbackButton;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Feedback buttons element](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element) representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::{types::FeedbackButton, FeedbackButtons};
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
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "feedback_buttons")]
pub struct FeedbackButtons {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[builder(validate("required"))]
    pub(crate) positive_button: Option<FeedbackButton>,

    #[builder(validate("required"))]
    pub(crate) negative_button: Option<FeedbackButton>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::elements::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = FeedbackButtons {
            action_id: Some("feedback_buttons_0".into()),
            positive_button: Some(fb_btn("Good", "positive")),
            negative_button: Some(fb_btn("Bad", "negative")),
        };

        let val = FeedbackButtons::builder()
            .set_action_id(Some("feedback_buttons_0"))
            .set_positive_button(Some(fb_btn("Good", "positive")))
            .set_negative_button(Some(fb_btn("Bad", "negative")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = FeedbackButtons::builder()
            .action_id("feedback_buttons_0")
            .positive_button(fb_btn("Good", "positive"))
            .negative_button(fb_btn("Bad", "negative"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = FeedbackButtons::builder()
            .action_id("a".repeat(256))
            .positive_button(fb_btn("Good", "positive"))
            .negative_button(fb_btn("Bad", "negative"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "FeedbackButtons");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_positive_button_field() {
        let err = FeedbackButtons::builder()
            .negative_button(fb_btn("Bad", "negative"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "FeedbackButtons");

        let errors = err.field("positive_button");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_negative_button_field() {
        let err = FeedbackButtons::builder()
            .positive_button(fb_btn("Good", "positive"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "FeedbackButtons");

        let errors = err.field("negative_button");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
