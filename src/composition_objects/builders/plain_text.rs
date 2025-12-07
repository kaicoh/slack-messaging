use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, PlainText};

use std::error::Error;
use std::fmt;

impl PlainText {
    /// Construct a [`PlainTextBuilder`].
    pub fn builder() -> PlainTextBuilder {
        PlainTextBuilder::default()
    }
}

/// Error while building [`PlainText`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct PlainTextError {
    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of emoji field
    pub emoji: Vec<ValidationError>,
}

impl fmt::Display for PlainTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PlainTextError {{ text: {:?}, emoji: {:?} }}",
            self.text, self.emoji,
        )
    }
}

impl Error for PlainTextError {}

/// Builder for [`PlainText`] object.
#[derive(Debug)]
pub struct PlainTextBuilder {
    text: Value<String>,
    emoji: Value<bool>,
}

impl Default for PlainTextBuilder {
    fn default() -> Self {
        PlainTextBuilder {
            text: new_text(None),
            emoji: new_emoji(None),
        }
    }
}

impl Builder for PlainTextBuilder {
    type Target = PlainText;
    type Error = PlainTextError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        value::merge_2(self.text, self.emoji)
            .map(|(text, emoji)| PlainText { text, emoji })
            .map_err(|(text, emoji)| PlainTextError { text, emoji })
    }
}

impl PlainTextBuilder {
    /// get text field value
    pub fn get_text(&self) -> Option<&String> {
        self.text.inner_ref()
    }

    /// set text field value
    pub fn set_text(self, text: Option<impl Into<String>>) -> Self {
        Self {
            text: new_text(text.map(|v| v.into())),
            ..self
        }
    }

    /// set text field value
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text))
    }

    /// get emoji field value
    pub fn get_emoji(&self) -> Option<bool> {
        self.emoji.inner_ref().copied()
    }

    /// set emoji field value
    pub fn set_emoji(self, emoji: Option<bool>) -> Self {
        Self {
            emoji: new_emoji(emoji),
            ..self
        }
    }

    /// set emoji field value
    pub fn emoji(self, emoji: bool) -> Self {
        self.set_emoji(Some(emoji))
    }
}

fn new_text(text: Option<String>) -> Value<String> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text::min_1 |
            validators::text::max_3000
    }
}

fn new_emoji(emoji: Option<bool>) -> Value<bool> {
    pipe! { Value::new(emoji) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = PlainText {
            text: Some("hello world".into()),
            emoji: Some(true),
        };

        let val = PlainText::builder()
            .set_text(Some("hello world"))
            .set_emoji(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = PlainText::builder()
            .text("hello world")
            .emoji(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn text_field_is_required() {
        let err = PlainText::builder().emoji(true).build().unwrap_err();

        let expected = PlainTextError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_must_be_more_than_1() {
        let err = PlainText::builder()
            .text("")
            .emoji(true)
            .build()
            .unwrap_err();

        let expected = PlainTextError {
            text: vec![ValidationError::MinTextLegth(1)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_must_be_less_than_3000() {
        let err = PlainText::builder()
            .text("a".repeat(3001))
            .emoji(true)
            .build()
            .unwrap_err();

        let expected = PlainTextError {
            text: vec![ValidationError::MaxTextLegth(3000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
