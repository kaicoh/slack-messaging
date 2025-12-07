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
#[derive(Debug, Clone, PartialEq)]
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::MrkdwnText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = MrkdwnText::builder()
    ///     .set_text(Some("hello world"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
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
    pub fn set_text(self, text: Option<impl Into<String>>) -> Self {
        Self {
            text: new_text(text.map(|v| v.into())),
            ..self
        }
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::MrkdwnText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = MrkdwnText::builder()
    ///     .text("hello world")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
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
        self.set_text(Some(text))
    }

    /// get verbatim field value
    pub fn get_verbatim(&self) -> Option<bool> {
        self.verbatim.inner_ref().copied()
    }

    /// set verbatim field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::MrkdwnText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = MrkdwnText::builder()
    ///     .text(":smile:")
    ///     .set_verbatim(Some(true))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
    ///     "text": ":smile:",
    ///     "verbatim": true,
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
    pub fn set_verbatim(self, verbatim: Option<bool>) -> Self {
        Self {
            verbatim: new_verbatim(verbatim),
            ..self
        }
    }

    /// set verbatim field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::MrkdwnText;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let text = MrkdwnText::builder()
    ///     .text(":smile:")
    ///     .verbatim(true)
    ///     .build()
    ///     .unwrap(); // unwrap Result::Ok
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
    ///     "text": ":smile:",
    ///     "verbatim": true,
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
    fn it_builds_mrkdwn() {
        let result = MrkdwnText::builder()
            .text("hello world")
            .verbatim(true)
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = MrkdwnText {
            text: Some("hello world".into()),
            verbatim: Some(true),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_builder_returns_text_required_error() {
        let result = MrkdwnText::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = MrkdwnTextError {
            text: vec![ValidationError::Required],
            verbatim: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_can_return_min_text_1_error() {
        let result = MrkdwnText::builder().text("").build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = MrkdwnTextError {
            text: vec![ValidationError::MinTextLegth(1)],
            verbatim: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_can_return_max_text_3000_error() {
        let result = MrkdwnText::builder().text("a".repeat(3001)).build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = MrkdwnTextError {
            text: vec![ValidationError::MaxTextLegth(3000)],
            verbatim: vec![],
        };
        assert_eq!(err, expected);
    }
}
