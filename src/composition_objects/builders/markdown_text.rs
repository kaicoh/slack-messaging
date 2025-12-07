use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, MrkdwnText};

use std::error::Error;
use std::fmt;

impl MrkdwnText {
    /// Construct a [`MrkdwnTextBuilder`].
    pub fn builder() -> MrkdwnTextBuilder {
        MrkdwnTextBuilder::default()
    }
}

/// Error while building [`MrkdwnText`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MrkdwnTextError {
    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of verbatim field
    pub verbatim: Vec<ValidationError>,
}

impl fmt::Display for MrkdwnTextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MrkdwnTextError {{ text: {:?}, verbatim: {:?} }}",
            self.text, self.verbatim,
        )
    }
}

impl Error for MrkdwnTextError {}

/// Builder for [`MrkdwnText`] object.
#[derive(Debug)]
pub struct MrkdwnTextBuilder {
    text: Value<String>,
    verbatim: Value<bool>,
}

impl Default for MrkdwnTextBuilder {
    fn default() -> Self {
        MrkdwnTextBuilder {
            text: new_text(None),
            verbatim: new_verbatim(None),
        }
    }
}

impl Builder for MrkdwnTextBuilder {
    type Target = MrkdwnText;
    type Error = MrkdwnTextError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        value::merge_2(self.text, self.verbatim)
            .map(|(text, verbatim)| MrkdwnText { text, verbatim })
            .map_err(|(text, verbatim)| MrkdwnTextError { text, verbatim })
    }
}

impl MrkdwnTextBuilder {
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

    /// get verbatim field value
    pub fn get_verbatim(&self) -> Option<bool> {
        self.verbatim.inner_ref().copied()
    }

    /// set verbatim field value
    pub fn set_verbatim(self, verbatim: Option<bool>) -> Self {
        Self {
            verbatim: new_verbatim(verbatim),
            ..self
        }
    }

    /// set verbatim field value
    pub fn verbatim(self, verbatim: bool) -> Self {
        self.set_verbatim(Some(verbatim))
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

fn new_verbatim(verbatim: Option<bool>) -> Value<bool> {
    pipe! { Value::new(verbatim) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = MrkdwnText {
            text: Some("hello world".into()),
            verbatim: Some(true),
        };

        let val = MrkdwnText::builder()
            .set_text(Some("hello world"))
            .set_verbatim(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MrkdwnText::builder()
            .text("hello world")
            .verbatim(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn text_field_is_required() {
        let err = MrkdwnText::builder().verbatim(true).build().unwrap_err();

        let expected = MrkdwnTextError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_must_be_more_than_1() {
        let err = MrkdwnText::builder()
            .text("")
            .verbatim(true)
            .build()
            .unwrap_err();

        let expected = MrkdwnTextError {
            text: vec![ValidationError::MinTextLegth(1)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_must_be_less_than_3000() {
        let err = MrkdwnText::builder()
            .text("a".repeat(3001))
            .verbatim(true)
            .build()
            .unwrap_err();

        let expected = MrkdwnTextError {
            text: vec![ValidationError::MaxTextLegth(3000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
