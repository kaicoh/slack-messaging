use crate::composition_objects::SlackFile;
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Image element](https://docs.slack.dev/reference/block-kit/block-elements/image-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::Image;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let image = Image::builder()
///     .image_url("http://placekitten.com/700/500")
///     .alt_text("Multiple cute kittens")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "image",
///     "image_url": "http://placekitten.com/700/500",
///     "alt_text": "Multiple cute kittens"
/// });
///
/// let json = serde_json::to_value(image).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let image = Image::builder()
///     .image_url("http://placekitten.com/700/500")
///     .build();
///
/// assert!(image.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "image")]
#[builder(validate = "validate")]
pub struct Image {
    #[builder(validate("required"))]
    pub(crate) alt_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_3000"))]
    pub(crate) image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slack_file: Option<SlackFile>,
}

fn validate(builder: &ImageBuilder) -> Vec<ValidationErrorKind> {
    match (
        builder.image_url.inner_ref(),
        builder.slack_file.inner_ref(),
    ) {
        (Some(_), Some(_)) => {
            vec![ValidationErrorKind::ExclusiveField(
                "image_url",
                "slack_file",
            )]
        }
        (None, None) => {
            vec![ValidationErrorKind::EitherRequired(
                "image_url",
                "slack_file",
            )]
        }
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;

    #[test]
    fn it_implements_builder() {
        // having image_url field
        let expected = Image {
            alt_text: Some("Cute kitten".into()),
            image_url: Some("http://placekitten.com/700/500".into()),
            slack_file: None,
        };

        let val = Image::builder()
            .set_alt_text(Some("Cute kitten"))
            .set_image_url(Some("http://placekitten.com/700/500"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("Cute kitten")
            .image_url("http://placekitten.com/700/500")
            .build()
            .unwrap();

        assert_eq!(val, expected);

        // having slack_file field
        let expected = Image {
            alt_text: Some("Cute kitten".into()),
            image_url: None,
            slack_file: Some(slack_file()),
        };

        let val = Image::builder()
            .set_alt_text(Some("Cute kitten"))
            .set_slack_file(Some(slack_file()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("Cute kitten")
            .slack_file(slack_file())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_alt_text_field() {
        let err = Image::builder()
            .slack_file(slack_file())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("alt_text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_image_url_less_than_3000_characters_long() {
        let err = Image::builder()
            .alt_text("Cute kitten")
            .image_url("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("image_url");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(3000)));
    }

    #[test]
    fn it_prevents_from_setting_both_image_url_and_slack_file() {
        let err = Image::builder()
            .alt_text("Cute kitten")
            .image_url("http://placekitten.com/700/500")
            .slack_file(slack_file())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::ExclusiveField(
            "image_url",
            "slack_file"
        )));
    }

    #[test]
    fn it_requires_either_image_url_or_slack_file_is_set() {
        let err = Image::builder()
            .alt_text("Cute kitten")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::EitherRequired(
            "image_url",
            "slack_file"
        )));
    }
}
