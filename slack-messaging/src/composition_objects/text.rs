use crate::validators::*;

use serde::{Serialize, Serializer};
use slack_messaging_derive::Builder;

/// [Text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// representation.
///
/// This is a generic struct that can represent either a plain text object or a markdown text
/// object, depending on the type parameter `T`.
///
/// # Type Parameters
///
/// * `T`: The type of text object. It can be either [`Plain`] for plain text or [`Mrkdwn`] for
///   markdown text.
///
/// # Fields and Validations
///
/// For more details, see the [Slack API
/// documentation](https://docs.slack.dev/reference/block-kit/composition-objects/text-object).
///
/// ## `Text<Plain>`
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | String | Yes | Minimum length: 1 character, Maximum length: 3000 characters. |
/// | emoji | bool | No | N/A |
///
/// ## `Text<Mrkdwn>`
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | String | Yes | Minimum length: 1 character, Maximum length: 3000 characters. |
/// | verbatim | bool | No | N/A |
///
/// # Example
///
///```
/// use slack_messaging::composition_objects::{Text, Plain, Mrkdwn};
/// # use std::error::Error;
/// # fn try_main() -> Result<(), Box<dyn Error>> {
///
/// // 1. Plain Text Object
/// let plain_text = Text::<Plain>::builder()
///     .text("Hello, World!")
///     .emoji(true)
///     .build()?;
///
/// let plain_json = serde_json::to_value(plain_text).unwrap();
///
/// let expected_plain = serde_json::json!({
///     "type": "plain_text",
///     "text": "Hello, World!",
///     "emoji": true
/// });
///
/// assert_eq!(plain_json, expected_plain);
///
/// // 2. Markdown Text Object
/// let mrkdwn_text = Text::<Mrkdwn>::builder()
///     .text("*Hello*, _World_!")
///     .verbatim(false)
///     .build()?;
///
/// let mrkdwn_json = serde_json::to_value(mrkdwn_text).unwrap();
///
/// let expected_mrkdwn = serde_json::json!({
///     "type": "mrkdwn",
///     "text": "*Hello*, _World_!",
///     "verbatim": false
/// });
///
/// assert_eq!(mrkdwn_json, expected_mrkdwn);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
///```
#[derive(Debug, Clone, Builder)]
pub struct Text<T> {
    #[builder(phantom = "T")]
    pub(crate) r#type: std::marker::PhantomData<T>,

    #[builder(validate("required", "text::min_1", "text::max_3000"))]
    pub(crate) text: Option<String>,

    #[builder(no_accessors)]
    pub(crate) emoji: Option<bool>, // for PlainText

    #[builder(no_accessors)]
    pub(crate) verbatim: Option<bool>, // for MrkdwnText
}

/// Extension trait for Text objects.
pub trait TextExt {
    fn text(&self) -> Option<&str>;
}

impl<T> TextExt for Text<T> {
    /// get text field value.
    fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }
}

/// Text object of type "plain_text".
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plain;

/// Text object of type "mrkdwn".
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mrkdwn;

impl TextBuilder<Plain> {
    /// get emoji field value.
    pub fn get_emoji(&self) -> Option<bool> {
        self.emoji.inner_ref().copied()
    }

    /// set emoji field value.
    pub fn set_emoji(self, emoji: Option<impl Into<bool>>) -> TextBuilder<Plain> {
        Self {
            emoji: Self::new_emoji(emoji.map(|v| v.into())),
            ..self
        }
    }

    /// set emoji field value.
    pub fn emoji(self, emoji: impl Into<bool>) -> TextBuilder<Plain> {
        self.set_emoji(Some(emoji))
    }
}

impl TextBuilder<Mrkdwn> {
    /// get verbatim field value.
    pub fn get_verbatim(&self) -> Option<bool> {
        self.verbatim.inner_ref().copied()
    }

    /// set verbatim field value.
    pub fn set_verbatim(self, verbatim: Option<impl Into<bool>>) -> TextBuilder<Mrkdwn> {
        Self {
            verbatim: Self::new_verbatim(verbatim.map(|v| v.into())),
            ..self
        }
    }

    /// set verbatim field value.
    pub fn verbatim(self, verbatim: impl Into<bool>) -> TextBuilder<Mrkdwn> {
        self.set_verbatim(Some(verbatim))
    }
}

impl PartialEq for Text<Plain> {
    fn eq(&self, other: &Self) -> bool {
        match (self.text(), other.text()) {
            (Some(text1), Some(text2)) if text1 != text2 => false,
            (None, Some(_)) | (Some(_), None) => false,
            _ => self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false),
        }
    }
}

impl PartialEq for Text<Mrkdwn> {
    fn eq(&self, other: &Self) -> bool {
        match (self.text(), other.text()) {
            (Some(text1), Some(text2)) if text1 != text2 => false,
            (None, Some(_)) | (Some(_), None) => false,
            _ => self.verbatim.unwrap_or(false) == other.verbatim.unwrap_or(false),
        }
    }
}

impl Serialize for Text<Plain> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Text", 3)?;

        state.serialize_field("type", "plain_text")?;

        if let Some(text) = &self.text {
            state.serialize_field("text", text)?;
        }

        if let Some(emoji) = &self.emoji {
            state.serialize_field("emoji", emoji)?;
        }

        state.end()
    }
}

impl Serialize for Text<Mrkdwn> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Text", 3)?;

        state.serialize_field("type", "mrkdwn")?;

        if let Some(text) = &self.text {
            state.serialize_field("text", text)?;
        }

        if let Some(verbatim) = &self.verbatim {
            state.serialize_field("verbatim", verbatim)?;
        }

        state.end()
    }
}

/// Enum representation of Text objects.
/// Use this when you need to handle both [`Plain`] and [`Mrkdwn`] text objects.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum TextContent {
    /// Plain text object.
    Plain(Text<Plain>),
    /// Markdown text object.
    Mrkdwn(Text<Mrkdwn>),
}

impl TextExt for TextContent {
    /// get text field value.
    fn text(&self) -> Option<&str> {
        match self {
            TextContent::Plain(t) => t.text(),
            TextContent::Mrkdwn(t) => t.text(),
        }
    }
}

impl From<Text<Plain>> for TextContent {
    fn from(text: Text<Plain>) -> Self {
        TextContent::Plain(text)
    }
}

impl From<Text<Mrkdwn>> for TextContent {
    fn from(text: Text<Mrkdwn>) -> Self {
        TextContent::Mrkdwn(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_requires_text() {
        let err = Text::<Plain>::builder().build().unwrap_err();
        assert_eq!(err.object(), "Text");
        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_more_than_1_character() {
        let err = Text::<Plain>::builder().text("").build().unwrap_err();
        assert_eq!(err.object(), "Text");
        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::MinTextLength(1)));
    }

    #[test]
    fn it_requires_text_less_than_3000_characters() {
        let err = Text::<Plain>::builder()
            .text("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Text");
        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::MaxTextLength(3000)));
    }

    mod plain_text {
        use super::*;

        #[test]
        fn it_implements_builder() {
            let expected = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello World:smile:".into()),
                emoji: Some(true),
                verbatim: None,
            };

            let text = Text::builder()
                .text("Hello World:smile:")
                .emoji(true)
                .build()
                .unwrap();

            assert_eq!(text, expected);

            let text = Text::builder()
                .set_text(Some("Hello World:smile:"))
                .set_emoji(Some(true))
                .build()
                .unwrap();

            assert_eq!(text, expected);
        }

        #[test]
        fn it_serializes_to_json() {
            let text = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello World :smile:".into()),
                emoji: Some(true),
                verbatim: None,
            };

            let expected = serde_json::json!({
                "type": "plain_text",
                "text": "Hello World :smile:",
                "emoji": true
            });
            let json = serde_json::to_value(&text).unwrap();
            assert_eq!(json, expected);
        }

        #[test]
        fn it_equals_another_plain_text() {
            let text1 = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello".into()),
                emoji: Some(true),
                verbatim: None,
            };
            let text2 = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello".into()),
                emoji: Some(true),
                verbatim: None,
            };
            let text3 = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello".into()),
                emoji: Some(false),
                verbatim: None,
            };
            let text4 = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("World".into()),
                emoji: Some(true),
                verbatim: None,
            };
            let text5 = Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello".into()),
                emoji: None,
                verbatim: None,
            };
            assert_eq!(text1, text2);
            assert_ne!(text1, text3);
            assert_ne!(text1, text4);
            assert_eq!(text3, text5);
        }
    }

    mod mrkdwn_text {
        use super::*;

        #[test]
        fn it_implements_builder() {
            let expected = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello* _World_ :smile:".into()),
                emoji: None,
                verbatim: Some(false),
            };

            let text = Text::builder()
                .text("*Hello* _World_ :smile:")
                .verbatim(false)
                .build()
                .unwrap();

            assert_eq!(text, expected);

            let text = Text::builder()
                .set_text(Some("*Hello* _World_ :smile:"))
                .set_verbatim(Some(false))
                .build()
                .unwrap();

            assert_eq!(text, expected);
        }

        #[test]
        fn it_serializes_to_json() {
            let text = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello* _World_ :smile:".into()),
                emoji: None,
                verbatim: Some(false),
            };

            let expected = serde_json::json!({
                "type": "mrkdwn",
                "text": "*Hello* _World_ :smile:",
                "verbatim": false
            });
            let json = serde_json::to_value(&text).unwrap();
            assert_eq!(json, expected);
        }

        #[test]
        fn it_equals_another_mrkdwn_text() {
            let text1 = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello*".into()),
                emoji: None,
                verbatim: Some(true),
            };
            let text2 = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello*".into()),
                emoji: None,
                verbatim: Some(true),
            };
            let text3 = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello*".into()),
                emoji: None,
                verbatim: Some(false),
            };
            let text4 = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("_World_".into()),
                emoji: None,
                verbatim: Some(true),
            };
            let text5 = Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello*".into()),
                emoji: None,
                verbatim: None,
            };
            assert_eq!(text1, text2);
            assert_ne!(text1, text3);
            assert_ne!(text1, text4);
            assert_eq!(text3, text5);
        }
    }

    mod text_content {
        use super::*;

        #[test]
        fn it_serializes_plain_text_variant_to_json() {
            let text = TextContent::from(Text::<Plain> {
                r#type: std::marker::PhantomData,
                text: Some("Hello World :smile:".into()),
                emoji: Some(true),
                verbatim: None,
            });

            let expected = serde_json::json!({
                "type": "plain_text",
                "text": "Hello World :smile:",
                "emoji": true
            });
            let json = serde_json::to_value(&text).unwrap();
            assert_eq!(json, expected);
        }

        #[test]
        fn it_serializes_mrkdwn_text_variant_to_json() {
            let text = TextContent::from(Text::<Mrkdwn> {
                r#type: std::marker::PhantomData,
                text: Some("*Hello* _World_ :smile:".into()),
                emoji: None,
                verbatim: Some(false),
            });

            let expected = serde_json::json!({
                "type": "mrkdwn",
                "text": "*Hello* _World_ :smile:",
                "verbatim": false
            });
            let json = serde_json::to_value(&text).unwrap();
            assert_eq!(json, expected);
        }
    }
}
