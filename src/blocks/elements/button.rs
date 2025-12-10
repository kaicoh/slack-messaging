use crate::composition_objects::{ConfirmationDialog, PlainText};
use crate::validators;
use crate::value::Value;

use derive_macro::Builder;
use serde::Serialize;

/// [Button element](https://docs.slack.dev/reference/block-kit/block-elements/button-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::Button;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = Button::builder()
///     .text(plain_text!("Click Me")?)
///     .value("click_me_123")
///     .action_id("button-0")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "button",
///     "text": {
///         "type": "plain_text",
///         "text": "Click Me"
///     },
///     "value": "click_me_123",
///     "action_id": "button-0"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = Button::builder()
///     .value("click_me_123")
///     .action_id("button-0")
///     .build();
///
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "button")]
pub struct Button {
    #[builder(setter = "set_text")]
    pub(crate) text: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter = "set_action_id")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter = "set_url")]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter = "set_value")]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(private_setter)]
    pub(crate) style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter = "set_accessibility_label")]
    pub(crate) accessibility_label: Option<String>,
}

impl ButtonBuilder {
    /// set "primary" to style field
    pub fn primary(self) -> Self {
        self.style("primary")
    }

    /// set "danger" to style field
    pub fn danger(self) -> Self {
        self.style("danger")
    }
}

fn set_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text_object::max_75
    }
}

fn set_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn set_url(url: Option<String>) -> Value<String> {
    pipe! { Value::new(url) => validators::text::max_3000 }
}

fn set_value(value: Option<String>) -> Value<String> {
    pipe! { Value::new(value) => validators::text::max_2000 }
}

fn set_accessibility_label(label: Option<String>) -> Value<String> {
    pipe! { Value::new(label) => validators::text::max_75 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Button {
            text: Some(plain_text("Click Me")),
            action_id: Some("button_0".into()),
            url: Some("https://docs.slack.dev/block-kit".into()),
            value: Some("click_me_123".into()),
            style: Some("primary"),
            confirm: Some(confirm()),
            accessibility_label: Some("Click Me!".into()),
        };

        let val = Button::builder()
            .set_text(Some(plain_text("Click Me")))
            .set_action_id(Some("button_0"))
            .set_url(Some("https://docs.slack.dev/block-kit"))
            .set_value(Some("click_me_123"))
            .primary()
            .set_confirm(Some(confirm()))
            .set_accessibility_label(Some("Click Me!"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let expected = Button {
            style: Some("danger"),
            ..expected
        };

        let val = Button::builder()
            .text(plain_text("Click Me"))
            .action_id("button_0")
            .url("https://docs.slack.dev/block-kit")
            .value("click_me_123")
            .danger()
            .confirm(confirm())
            .accessibility_label("Click Me!")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = Button::builder().build().unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_less_than_75_characters_long() {
        let err = Button::builder()
            .text(plain_text("a".repeat(76)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = Button::builder()
            .text(plain_text("Click Me"))
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_url_less_than_3000_characters_long() {
        let err = Button::builder()
            .text(plain_text("Click Me"))
            .url("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("url");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(3000)));
    }

    #[test]
    fn it_requires_value_less_than_2000_characters_long() {
        let err = Button::builder()
            .text(plain_text("Click Me"))
            .value("a".repeat(2001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(2000)));
    }

    #[test]
    fn it_requires_accessibility_label_less_than_75_characters_long() {
        let err = Button::builder()
            .text(plain_text("Click Me"))
            .accessibility_label("a".repeat(76))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Button");

        let errors = err.field("accessibility_label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
    }
}
