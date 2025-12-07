use super::composition_objects::PlainText;
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, FeedbackButton, FeedbackButtons};

use std::error::Error;
use std::fmt;

impl FeedbackButton {
    /// Construct a [`FeedbackButtonBuilder`].
    pub fn builder() -> FeedbackButtonBuilder {
        FeedbackButtonBuilder::default()
    }
}

/// Error while building [`FeedbackButton`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeedbackButtonError {
    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of value field
    pub value: Vec<ValidationError>,

    /// errors of accessibility_label field
    pub accessibility_label: Vec<ValidationError>,
}

impl fmt::Display for FeedbackButtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeedbackButtonError {{ text: {:?}, value: {:?}, accessibility_label: {:?} }}",
            self.text, self.value, self.accessibility_label,
        )
    }
}

impl Error for FeedbackButtonError {}

/// Builder for [`FeedbackButton`] object.
#[derive(Debug)]
pub struct FeedbackButtonBuilder {
    text: Value<PlainText>,
    value: Value<String>,
    accessibility_label: Value<String>,
}

impl Default for FeedbackButtonBuilder {
    fn default() -> Self {
        FeedbackButtonBuilder {
            text: new_text(None),
            value: new_value(None),
            accessibility_label: new_accessibility_label(None),
        }
    }
}

impl Builder for FeedbackButtonBuilder {
    type Target = FeedbackButton;
    type Error = FeedbackButtonError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            text,
            value,
            accessibility_label,
        } = self;
        value::merge_3(text, value, accessibility_label)
            .map(|(text, value, accessibility_label)| FeedbackButton {
                text,
                value,
                accessibility_label,
            })
            .map_err(|(text, value, accessibility_label)| FeedbackButtonError {
                text,
                value,
                accessibility_label,
            })
    }
}

impl FeedbackButtonBuilder {
    /// get text field value
    pub fn get_text(&self) -> Option<&PlainText> {
        self.text.inner_ref()
    }

    /// set text field value
    pub fn set_text(self, text: Option<PlainText>) -> Self {
        Self {
            text: new_text(text),
            ..self
        }
    }

    /// set text field value
    pub fn text(self, text: PlainText) -> Self {
        self.set_text(Some(text))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&String> {
        self.value.inner_ref()
    }

    /// set value field value
    pub fn set_value(self, value: Option<impl Into<String>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value))
    }

    /// get accessibility_label field value
    pub fn get_accessibility_label(&self) -> Option<&String> {
        self.accessibility_label.inner_ref()
    }

    /// set accessibility_label field value
    pub fn set_accessibility_label(self, label: Option<impl Into<String>>) -> Self {
        Self {
            accessibility_label: new_accessibility_label(label.map(|v| v.into())),
            ..self
        }
    }

    /// set accessibility_label field value
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label))
    }
}

fn new_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text_object::max_75
    }
}

fn new_value(value: Option<String>) -> Value<String> {
    pipe! {
        Value::new(value) =>
            validators::required |
            validators::text::max_2000
    }
}

fn new_accessibility_label(label: Option<String>) -> Value<String> {
    pipe! { Value::new(label) => validators::text::max_75 }
}

impl FeedbackButtons {
    /// Construct a [`FeedbackButtonsBuilder`].
    pub fn builder() -> FeedbackButtonsBuilder {
        FeedbackButtonsBuilder::default()
    }
}

/// Error while building [`FeedbackButtons`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FeedbackButtonsError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of positive_button field
    pub positive_button: Vec<ValidationError>,

    /// errors of negative_button field
    pub negative_button: Vec<ValidationError>,
}

impl fmt::Display for FeedbackButtonsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FeedbackButtonsError {{ action_id: {:?}, positive_button: {:?}, negative_button: {:?} }}",
            self.action_id, self.positive_button, self.negative_button,
        )
    }
}

impl Error for FeedbackButtonsError {}

/// Builder for [`FeedbackButtons`] object.
#[derive(Debug)]
pub struct FeedbackButtonsBuilder {
    action_id: Value<String>,
    positive_button: Value<FeedbackButton>,
    negative_button: Value<FeedbackButton>,
}

impl Default for FeedbackButtonsBuilder {
    fn default() -> Self {
        FeedbackButtonsBuilder {
            action_id: new_action_id(None),
            positive_button: new_positive_button(None),
            negative_button: new_negative_button(None),
        }
    }
}

impl Builder for FeedbackButtonsBuilder {
    type Target = FeedbackButtons;
    type Error = FeedbackButtonsError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            positive_button,
            negative_button,
        } = self;
        value::merge_3(action_id, positive_button, negative_button)
            .map(
                |(action_id, positive_button, negative_button)| FeedbackButtons {
                    action_id,
                    positive_button,
                    negative_button,
                },
            )
            .map_err(
                |(action_id, positive_button, negative_button)| FeedbackButtonsError {
                    action_id,
                    positive_button,
                    negative_button,
                },
            )
    }
}

impl FeedbackButtonsBuilder {
    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    pub fn set_action_id(self, action_id: Option<impl Into<String>>) -> Self {
        Self {
            action_id: new_action_id(action_id.map(|v| v.into())),
            ..self
        }
    }

    /// set action_id field value
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id))
    }

    /// get positive_button field value
    pub fn get_positive_button(&self) -> Option<&FeedbackButton> {
        self.positive_button.inner_ref()
    }

    /// set positive_button field value
    pub fn set_positive_button(self, button: Option<FeedbackButton>) -> Self {
        Self {
            positive_button: new_positive_button(button),
            ..self
        }
    }

    /// set positive_button field value
    pub fn positive_button(self, button: FeedbackButton) -> Self {
        self.set_positive_button(Some(button))
    }

    /// get negative_button field value
    pub fn get_negative_button(&self) -> Option<&FeedbackButton> {
        self.negative_button.inner_ref()
    }

    /// set negative_button field value
    pub fn set_negative_button(self, button: Option<FeedbackButton>) -> Self {
        Self {
            negative_button: new_negative_button(button),
            ..self
        }
    }

    /// set negative_button field value
    pub fn negative_button(self, button: FeedbackButton) -> Self {
        self.set_negative_button(Some(button))
    }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_positive_button(button: Option<FeedbackButton>) -> Value<FeedbackButton> {
    pipe! { Value::new(button) => validators::required }
}

fn new_negative_button(button: Option<FeedbackButton>) -> Value<FeedbackButton> {
    pipe! { Value::new(button) => validators::required }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::test_helpers::*;
    use super::*;

    #[test]
    fn feedback_button_builder_has_setter_methods() {
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
    fn text_field_of_feedback_button_builder_is_required() {
        let err = FeedbackButton::builder()
            .value("positive_feedback")
            .build()
            .unwrap_err();

        let expected = FeedbackButtonError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_of_feedback_button_builder_must_be_less_than_75() {
        let err = FeedbackButton::builder()
            .text(plain_text("a".repeat(76)))
            .value("positive_feedback")
            .build()
            .unwrap_err();

        let expected = FeedbackButtonError {
            text: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_of_feedback_button_builder_is_required() {
        let err = FeedbackButton::builder()
            .text(plain_text("Good"))
            .build()
            .unwrap_err();

        let expected = FeedbackButtonError {
            value: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_length_of_feedback_button_builder_must_be_less_than_2000() {
        let err = FeedbackButton::builder()
            .text(plain_text("Good"))
            .value("a".repeat(2001))
            .build()
            .unwrap_err();

        let expected = FeedbackButtonError {
            value: vec![ValidationError::MaxTextLegth(2000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn accessibility_label_field_length_of_feedback_button_builder_must_be_less_than_75() {
        let err = FeedbackButton::builder()
            .text(plain_text("Gook"))
            .value("positive_feedback")
            .accessibility_label("a".repeat(76))
            .build()
            .unwrap_err();

        let expected = FeedbackButtonError {
            accessibility_label: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn feedback_buttons_builder_has_setter_methods() {
        let expected = FeedbackButtons {
            action_id: Some("feedback_buttons-0".into()),
            positive_button: Some(positive_button()),
            negative_button: Some(negative_button()),
        };

        let val = FeedbackButtons::builder()
            .set_action_id(Some("feedback_buttons-0"))
            .set_positive_button(Some(positive_button()))
            .set_negative_button(Some(negative_button()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = FeedbackButtons::builder()
            .action_id("feedback_buttons-0")
            .positive_button(positive_button())
            .negative_button(negative_button())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_of_feedback_buttons_builder_must_be_less_than_255() {
        let err = FeedbackButtons::builder()
            .action_id("a".repeat(256))
            .positive_button(positive_button())
            .negative_button(negative_button())
            .build()
            .unwrap_err();

        let expected = FeedbackButtonsError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn positive_button_field_length_of_feedback_buttons_builder_is_required() {
        let err = FeedbackButtons::builder()
            .action_id("feedback_buttons-0")
            .negative_button(negative_button())
            .build()
            .unwrap_err();

        let expected = FeedbackButtonsError {
            positive_button: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn negative_button_field_length_of_feedback_buttons_builder_is_required() {
        let err = FeedbackButtons::builder()
            .action_id("feedback_buttons-0")
            .positive_button(positive_button())
            .build()
            .unwrap_err();

        let expected = FeedbackButtonsError {
            negative_button: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    fn positive_button() -> FeedbackButton {
        FeedbackButton {
            text: Some(plain_text("Good")),
            value: Some("positive_feedback".into()),
            accessibility_label: None,
        }
    }

    fn negative_button() -> FeedbackButton {
        FeedbackButton {
            text: Some(plain_text("Bad")),
            value: Some("nagative_feedback".into()),
            accessibility_label: None,
        }
    }
}
