use super::super::option::UrlAvailable;
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Opt, TextInOption};

use std::error::Error;
use std::fmt;
use std::marker::PhantomData;

impl<T: TextInOption, P> Opt<T, P> {
    /// Construct a [`OptBuilder`].
    pub fn builder() -> OptBuilder<T, P> {
        OptBuilder::default()
    }
}

/// Error while building [`Opt`] object.
#[derive(Debug, Clone, PartialEq, Default)]
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
pub struct OptBuilder<T: TextInOption, P> {
    phantom: PhantomData<P>,
    text: Value<T>,
    value: Value<String>,
    description: Value<T>,
    url: Value<String>,
}

impl<T: TextInOption, P> Default for OptBuilder<T, P> {
    fn default() -> Self {
        OptBuilder {
            phantom: PhantomData,
            text: new_text(None),
            value: new_value(None),
            description: new_description(None),
            url: new_url(None),
        }
    }
}

impl<T: TextInOption, P> Builder for OptBuilder<T, P> {
    type Target = Opt<T, P>;
    type Error = OptError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            phantom,
            text,
            value,
            description,
            url,
        } = self;

        value::merge_4(text, value, description, url)
            .map(|(text, value, description, url)| Opt {
                phantom,
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

impl<T: TextInOption, P> OptBuilder<T, P> {
    /// get text field value
    pub fn get_text(&self) -> Option<&T> {
        self.text.inner_ref()
    }

    /// set text field value
    pub fn set_text(self, text: Option<impl Into<T>>) -> Self {
        Self {
            text: new_text(text.map(|v| v.into())),
            ..self
        }
    }

    /// set text field value
    pub fn text(self, text: impl Into<T>) -> Self {
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

    /// get description field value
    pub fn get_description(&self) -> Option<&T> {
        self.description.inner_ref()
    }

    /// set description field value
    pub fn set_description(self, description: Option<impl Into<T>>) -> Self {
        Self {
            description: new_description(description.map(|v| v.into())),
            ..self
        }
    }

    /// set description field value
    pub fn description(self, description: impl Into<T>) -> Self {
        self.set_description(Some(description))
    }
}

impl<T: TextInOption> OptBuilder<T, UrlAvailable> {
    /// get url field value
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value
    pub fn set_url(self, url: Option<impl Into<String>>) -> Self {
        Self {
            url: new_url(url.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value
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
    use super::super::{PlainText, test_helpers::*};
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = Opt {
            phantom: PhantomData,
            text: Some(plain_text("foo")),
            value: Some("bar".into()),
            description: Some(plain_text("baz")),
            url: None,
        };

        let val = Opt::<PlainText>::builder()
            .set_text(Some(plain_text("foo")))
            .set_value(Some("bar"))
            .set_description(Some(plain_text("baz")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn url_field_is_available_only_if_the_phantom_type_is_used() {
        let expected = Opt {
            phantom: PhantomData,
            text: Some(plain_text("foo")),
            value: Some("bar".into()),
            description: Some(plain_text("baz")),
            url: Some("foobarbaz".into()),
        };

        let val = Opt::<PlainText, UrlAvailable>::builder()
            .set_text(Some(plain_text("foo")))
            .set_value(Some("bar"))
            .set_description(Some(plain_text("baz")))
            .set_url(Some("foobarbaz"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Opt::<PlainText, UrlAvailable>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .url("foobarbaz")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn text_field_is_required() {
        let err = Opt::<PlainText>::builder()
            .value("bar")
            .build()
            .unwrap_err();

        let expected = OptError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_must_be_less_than_75() {
        let err = Opt::<PlainText>::builder()
            .text(plain_text("f".repeat(76)))
            .value("bar")
            .build()
            .unwrap_err();

        let expected = OptError {
            text: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_is_required() {
        let err = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .build()
            .unwrap_err();

        let expected = OptError {
            value: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_length_must_be_less_than_150() {
        let err = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("b".repeat(151))
            .build()
            .unwrap_err();

        let expected = OptError {
            value: vec![ValidationError::MaxTextLegth(150)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn description_field_length_must_be_less_than_75() {
        let err = Opt::<PlainText>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("b".repeat(76)))
            .build()
            .unwrap_err();

        let expected = OptError {
            description: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn url_field_length_must_be_less_than_3000() {
        let err = Opt::<PlainText, UrlAvailable>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .url("f".repeat(3001))
            .build()
            .unwrap_err();

        let expected = OptError {
            url: vec![ValidationError::MaxTextLegth(3000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
