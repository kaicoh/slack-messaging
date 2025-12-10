use crate::composition_objects::PlainText;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// Button object to be set to the `positive_buttons` and `negative_buttons`
/// fields of [`FeedbackButtons`](crate::blocks::elements::FeedbackButtons) object.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::types::FeedbackButton;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = FeedbackButton::builder()
///     .text(plain_text!("Good")?)
///     .value("positive_feedback")
///     .accessibility_label("Mark this response as good")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Good"
///     },
///     "value": "positive_feedback",
///     "accessibility_label": "Mark this response as good"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = FeedbackButton::builder()
///     .text(plain_text!("Good")?)
///     .build();
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct FeedbackButton {
    #[builder(validate("required", "text_object::max_75"))]
    pub(crate) text: Option<PlainText>,

    #[builder(validate("required", "text::max_2000"))]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_75"))]
    pub(crate) accessibility_label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    mod feedback_button {
        use super::*;

        #[test]
        fn it_implements_builder() {
            let expected = FeedbackButton {
                text: Some(plain_text("Good")),
                value: Some("positive_feedback".into()),
                accessibility_label: Some("Mark this response as good".into()),
            };

            let val = FeedbackButton::builder()
                .set_text(Some(plain_text("Good")))
                .set_value(Some("positive_feedback"))
                .set_accessibility_label(Some("Mark this response as good"))
                .build()
                .unwrap();

            assert_eq!(val, expected);

            let val = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("positive_feedback")
                .accessibility_label("Mark this response as good")
                .build()
                .unwrap();

            assert_eq!(val, expected);
        }

        #[test]
        fn it_requires_text_field() {
            let err = FeedbackButton::builder()
                .value("positive_feedback")
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("text");
            assert!(errors.includes(ValidationErrorKind::Required));
        }

        #[test]
        fn it_requires_text_less_than_75_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("a".repeat(76)))
                .value("positive_feedback")
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("text");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
        }

        #[test]
        fn it_requires_value_field() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("value");
            assert!(errors.includes(ValidationErrorKind::Required));
        }

        #[test]
        fn it_requires_value_less_than_2000_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("a".repeat(2001))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("value");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(2000)));
        }

        #[test]
        fn it_requires_accessibility_label_less_than_75_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("positive_feedback")
                .accessibility_label("a".repeat(76))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("accessibility_label");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
        }
    }
}
