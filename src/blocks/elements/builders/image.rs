use super::composition_objects::SlackFile;
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Image};

use std::error::Error;
use std::fmt;

impl Image {
    /// Construct a [`ImageBuilder`].
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }
}

/// Error while building [`Image`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ImageError {
    /// errors of alt_text field
    pub alt_text: Vec<ValidationError>,

    /// errors of image_url field
    pub image_url: Vec<ValidationError>,

    /// errors of slack_file field
    pub slack_file: Vec<ValidationError>,
}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ImageError {{ alt_text: {:?}, image_url: {:?}, slack_file: {:?} }}",
            self.alt_text, self.image_url, self.slack_file
        )
    }
}

impl Error for ImageError {}

/// Builder for [`Image`] object.
#[derive(Debug)]
pub struct ImageBuilder {
    alt_text: Value<String>,
    image_url: Value<String>,
    slack_file: Value<SlackFile>,
}

impl Default for ImageBuilder {
    fn default() -> Self {
        ImageBuilder {
            alt_text: new_alt_text(None),
            image_url: new_image_url(None),
            slack_file: new_slack_file(None),
        }
    }
}

impl Builder for ImageBuilder {
    type Target = Image;
    type Error = ImageError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            alt_text,
            image_url,
            slack_file,
        } = self;
        value::merge_3(alt_text, image_url, slack_file)
            .map(|(alt_text, image_url, slack_file)| Image {
                alt_text,
                image_url,
                slack_file,
            })
            .map_err(|(alt_text, image_url, slack_file)| ImageError {
                alt_text,
                image_url,
                slack_file,
            })
    }
}

impl ImageBuilder {
    /// get alt_text field value
    pub fn get_alt_text(&self) -> Option<&String> {
        self.alt_text.inner_ref()
    }

    /// set alt_text field value
    pub fn set_alt_text(self, alt_text: Option<impl Into<String>>) -> Self {
        Self {
            alt_text: new_alt_text(alt_text.map(|v| v.into())),
            ..self
        }
    }

    /// set alt_text field value
    pub fn alt_text(self, alt_text: impl Into<String>) -> Self {
        self.set_alt_text(Some(alt_text))
    }

    /// get image_url field value
    pub fn get_image_url(&self) -> Option<&String> {
        self.image_url.inner_ref()
    }

    /// set image_url field value
    pub fn set_image_url(self, image_url: Option<impl Into<String>>) -> Self {
        Self {
            image_url: new_image_url(image_url.map(|v| v.into())),
            ..self
        }
    }

    /// set image_url field value
    pub fn image_url(self, image_url: impl Into<String>) -> Self {
        self.set_image_url(Some(image_url))
    }

    /// get slack_file field value
    pub fn get_slack_file(&self) -> Option<&SlackFile> {
        self.slack_file.inner_ref()
    }

    /// set slack_file field value
    pub fn set_slack_file(self, slack_file: Option<SlackFile>) -> Self {
        Self {
            slack_file: new_slack_file(slack_file),
            ..self
        }
    }

    /// set slack_file field value
    pub fn slack_file(self, slack_file: SlackFile) -> Self {
        self.set_slack_file(Some(slack_file))
    }
}

fn new_alt_text(alt_text: Option<String>) -> Value<String> {
    pipe! { Value::new(alt_text) => validators::required }
}

fn new_image_url(options: Option<String>) -> Value<String> {
    pipe! { Value::new(options) => validators::text::max_3000 }
}

fn new_slack_file(slack_file: Option<SlackFile>) -> Value<SlackFile> {
    pipe! { Value::new(slack_file) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        // use image_url field
        let expected = Image {
            alt_text: Some("Multiple cute kittens".into()),
            image_url: Some("http://placekitten.com/700/500".into()),
            slack_file: None,
        };

        let val = Image::builder()
            .set_alt_text(Some("Multiple cute kittens"))
            .set_image_url(Some("http://placekitten.com/700/500"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("Multiple cute kittens")
            .image_url("http://placekitten.com/700/500")
            .build()
            .unwrap();

        assert_eq!(val, expected);

        // use slack_file field
        let expected = Image {
            alt_text: Some("Multiple cute kittens".into()),
            image_url: None,
            slack_file: Some(slack_file()),
        };

        let val = Image::builder()
            .set_alt_text(Some("Multiple cute kittens"))
            .set_slack_file(Some(slack_file()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("Multiple cute kittens")
            .slack_file(slack_file())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn alt_text_field_is_required() {
        let err = Image::builder().build().unwrap_err();

        let expected = ImageError {
            alt_text: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn image_url_field_length_must_be_less_than_3000() {
        let err = Image::builder()
            .alt_text("Multiple cute kittens")
            .image_url("a".repeat(3001))
            .build()
            .unwrap_err();

        let expected = ImageError {
            image_url: vec![ValidationError::MaxTextLegth(3000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
