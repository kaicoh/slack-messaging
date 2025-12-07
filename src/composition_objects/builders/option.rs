use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Opt, TextInOption};

use std::error::Error;
use std::fmt;

impl<T: TextInOption> Opt<T> {
    /// Construct a [`OptBuilder`].
    pub fn builder() -> OptBuilder<T> {
        OptBuilder::default()
    }
}

/// Error while building [`Opt`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct OptError {
    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of value field
    pub value: Vec<ValidationError>,

    /// errors of description field
    pub description: Vec<ValidationError>,

    /// errors of url field
    pub url: Vec<ValidationError>,
}

impl fmt::Display for OptError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OptError {{ text: {:?}, value: {:?}, description: {:?}, url: {:?} }}",
            self.text, self.value, self.description, self.url
        )
    }
}

impl Error for OptError {}

/// Builder for [`Opt`] object.
#[derive(Debug)]
pub struct OptBuilder<T: TextInOption> {
    text: Value<T>,
    value: Value<String>,
    description: Value<T>,
    url: Value<String>,
}

impl<T: TextInOption> Default for OptBuilder<T> {
    fn default() -> Self {
        OptBuilder {
            text: new_text(None),
            value: new_value(None),
            description: new_description(None),
            url: new_url(None),
        }
    }
}

impl<T: TextInOption> Builder for OptBuilder<T> {
    type Target = Opt<T>;
    type Error = OptError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            text,
            value,
            description,
            url,
        } = self;

        value::merge_4(text, value, description, url)
            .map(|(text, value, description, url)| Opt {
                text,
                value,
                description,
                url,
            })
            .map_err(|(text, value, description, url)| OptError {
                text,
                value,
                description,
                url,
            })
    }
}

impl<T: TextInOption> OptBuilder<T> {
    /// get text field value
    pub fn get_text(&self) -> Option<&T> {
        self.text.inner_ref()
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .set_text(Some(plain_text!("Maru")?))
    ///     .value("maru")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn set_text(self, text: Option<impl Into<T>>) -> Self {
        Self {
            text: new_text(text.map(|v| v.into())),
            ..self
        }
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn text(self, text: impl Into<T>) -> Self {
        self.set_text(Some(text))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&String> {
        self.value.inner_ref()
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .set_value(Some("maru"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn set_value(self, value: Option<impl Into<String>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value))
    }

    /// get description field value
    pub fn get_description(&self) -> Option<&T> {
        self.description.inner_ref()
    }

    /// set description field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .set_description(Some(plain_text!("This is a description.")?))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru",
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn set_description(self, description: Option<impl Into<T>>) -> Self {
        Self {
            description: new_description(description.map(|v| v.into())),
            ..self
        }
    }

    /// set description field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .description(plain_text!("This is a description.")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru",
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn description(self, description: impl Into<T>) -> Self {
        self.set_description(Some(description))
    }

    /// get url field value
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .set_url(Some("https://google.com"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn set_url(self, url: Option<impl Into<String>>) -> Self {
        Self {
            url: new_url(url.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("Maru")?)
    ///     .value("maru")
    ///     .url("https://google.com")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Maru"
    ///     },
    ///     "value": "maru",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url))
    }
}

fn new_text<T: TextInOption>(text: Option<T>) -> Value<T> {
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
            validators::text::max_150
    }
}

fn new_description<T: TextInOption>(description: Option<T>) -> Value<T> {
    pipe! { Value::new(description) => validators::text_object::max_75 }
}

fn new_url(url: Option<String>) -> Value<String> {
    pipe! { Value::new(url) => validators::text::max_3000 }
}

#[cfg(test)]
mod tests {
    use super::super::PlainText;
    use super::*;

    #[test]
    fn it_builds_option() {
        let result = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .url("foobarbaz")
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = Opt {
            text: Some(plain_text("foo")),
            value: Some("bar".into()),
            description: Some(plain_text("baz")),
            url: Some("foobarbaz".into()),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_builder_returns_error() {
        let result = Opt::<PlainText>::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptError {
            text: vec![ValidationError::Required],
            value: vec![ValidationError::Required],
            description: vec![],
            url: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_75_error_if_text_length_is_more_than_75() {
        let result = Opt::<PlainText>::builder()
            .text(plain_text("f".repeat(76)))
            .value("bar")
            .description(plain_text("baz"))
            .url("foobarbaz")
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptError {
            text: vec![ValidationError::MaxTextLegth(75)],
            value: vec![],
            description: vec![],
            url: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_150_error_if_value_length_is_more_than_150() {
        let result = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("b".repeat(151))
            .description(plain_text("baz"))
            .url("foobarbaz")
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptError {
            text: vec![],
            value: vec![ValidationError::MaxTextLegth(150)],
            description: vec![],
            url: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_75_error_if_description_length_is_more_than_75() {
        let result = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("b".repeat(76)))
            .url("foobarbaz")
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptError {
            text: vec![],
            value: vec![],
            description: vec![ValidationError::MaxTextLegth(75)],
            url: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_3000_error_if_url_length_is_more_than_3000() {
        let result = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .url("f".repeat(3001))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptError {
            text: vec![],
            value: vec![],
            description: vec![],
            url: vec![ValidationError::MaxTextLegth(3000)],
        };
        assert_eq!(err, expected);
    }

    fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
