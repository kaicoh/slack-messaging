use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// Plain [text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// representation.
///
/// # Example
///
///```
/// use slack_messaging::composition_objects::PlainText;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let text = PlainText::builder()
///     .text("Hello, World!")
///     .build()?;
///
/// let json = serde_json::to_value(text).unwrap();
///
/// let expected = serde_json::json!({
///     "type": "plain_text",
///     "text": "Hello, World!"
/// });
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let text = PlainText::builder()
///     .text("")
///     .build();
///
/// assert!(text.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
///```
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(tag = "type", rename = "plain_text")]
pub struct PlainText {
    #[builder(validate("required", "text::min_1", "text::max_3000"))]
    pub(crate) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) emoji: Option<bool>,
}

impl PartialEq for PlainText {
    fn eq(&self, other: &Self) -> bool {
        match (self.text.as_ref(), other.text.as_ref()) {
            (Some(t0), Some(t1)) => {
                t0.as_str() == t1.as_str()
                    && self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = PlainText {
            text: Some("Hello World:smile:".into()),
            emoji: Some(true),
        };

        let text = PlainText::builder()
            .text("Hello World:smile:")
            .emoji(true)
            .build()
            .unwrap();

        assert_eq!(text, expected);

        let text = PlainText::builder()
            .set_text(Some("Hello World:smile:"))
            .set_emoji(Some(true))
            .build()
            .unwrap();

        assert_eq!(text, expected);
    }

    #[test]
    fn it_requires_text() {
        let err = PlainText::builder().build().unwrap_err();
        assert_eq!(err.object(), "PlainText");

        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_more_than_1_character() {
        let err = PlainText::builder().text("").build().unwrap_err();
        assert_eq!(err.object(), "PlainText");

        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::MinTextLegth(1)));
    }

    #[test]
    fn it_requires_text_less_than_3000_characters() {
        let err = PlainText::builder()
            .text("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PlainText");

        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::MaxTextLegth(3000)));
    }

    #[test]
    fn it_equals_with_same_text() {
        let t_0 = make_text("Hello");
        let t_1 = make_text("Hello");
        let t_2 = make_text("hello");

        assert_eq!(t_0, t_1);
        assert_ne!(t_0, t_2);
    }

    #[test]
    fn it_compares_emoji_field_when_plain_text() {
        let t_0 = make_text("Hello");
        let t_1 = PlainText {
            emoji: Some(false),
            ..make_text("Hello")
        };
        let t_2 = PlainText {
            emoji: Some(true),
            ..make_text("Hello")
        };

        assert_eq!(t_0, t_1);
        assert_ne!(t_0, t_2);
    }

    fn make_text(text: &str) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
