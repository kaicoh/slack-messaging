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
#[derive(Debug, Clone, PartialEq)]
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::PlainText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = PlainText::builder()
    ///     .set_text(Some("hello world".into()))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self {
            text: new_text(text),
            ..self
        }
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::PlainText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = PlainText::builder()
    ///     .text("hello world")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// get emoji field value
    pub fn get_emoji(&self) -> Option<bool> {
        self.emoji.inner_ref().copied()
    }

    /// set emoji field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::PlainText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = PlainText::builder()
    ///     .text(":smile:")
    ///     .set_emoji(Some(true))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": ":smile:",
    ///     "emoji": true,
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_emoji(self, emoji: Option<bool>) -> Self {
        Self {
            emoji: new_emoji(emoji),
            ..self
        }
    }

    /// set emoji field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::PlainText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = PlainText::builder()
    ///     .text(":smile:")
    ///     .emoji(true)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": ":smile:",
    ///     "emoji": true,
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
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
    fn it_builds_plain_text() {
        let result = PlainText::builder().text("hello world").emoji(true).build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = PlainText {
            text: Some("hello world".into()),
            emoji: Some(true),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_builder_returns_text_required_error() {
        let result = PlainText::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = PlainTextError {
            text: vec![ValidationError::Required],
            emoji: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_can_return_min_text_1_error() {
        let result = PlainText::builder().text("").build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = PlainTextError {
            text: vec![ValidationError::MinTextLegth(1)],
            emoji: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_can_return_max_text_3000_error() {
        let result = PlainText::builder().text("a".repeat(3001)).build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = PlainTextError {
            text: vec![ValidationError::MaxTextLegth(3000)],
            emoji: vec![],
        };
        assert_eq!(err, expected);
    }
}
