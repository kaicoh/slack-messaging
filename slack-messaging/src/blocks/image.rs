use crate::composition_objects::{PlainText, SlackFile};
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Image block](https://docs.slack.dev/reference/block-kit/blocks/image-block)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Image;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let image = Image::builder()
///     .block_id("image4")
///     .title(plain_text!("Please enjoy this photo of a kitten")?)
///     .image_url("http://placekitten.com/500/500")
///     .alt_text("An incredibly cute kitten.")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "image",
///     "block_id": "image4",
///     "title": {
///         "type": "plain_text",
///         "text": "Please enjoy this photo of a kitten"
///     },
///     "image_url": "http://placekitten.com/500/500",
///     "alt_text": "An incredibly cute kitten."
/// });
///
/// let json = serde_json::to_value(image).unwrap();
///
/// assert_eq!(json, expected);
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
    #[builder(validate("required", "text::max_2000"))]
    pub(crate) alt_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_3000"))]
    pub(crate) image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_2000"))]
    pub(crate) title: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

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
        // using image_url
        let expected = Image {
            alt_text: Some("An incredibly cute kitten.".into()),
            image_url: Some("http://placekitten.com/500/500".into()),
            title: Some(plain_text("Please enjoy this photo of a kitten")),
            block_id: Some("image4".into()),
            slack_file: None,
        };

        let val = Image::builder()
            .set_alt_text(Some("An incredibly cute kitten."))
            .set_image_url(Some("http://placekitten.com/500/500"))
            .set_title(Some(plain_text("Please enjoy this photo of a kitten")))
            .set_block_id(Some("image4"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("An incredibly cute kitten.")
            .image_url("http://placekitten.com/500/500")
            .title(plain_text("Please enjoy this photo of a kitten"))
            .block_id("image4")
            .build()
            .unwrap();

        assert_eq!(val, expected);

        // using slack file
        let expected = Image {
            alt_text: Some("An incredibly cute kitten.".into()),
            image_url: None,
            title: Some(plain_text("Please enjoy this photo of a kitten")),
            block_id: Some("image4".into()),
            slack_file: Some(slack_file()),
        };

        let val = Image::builder()
            .set_alt_text(Some("An incredibly cute kitten."))
            .set_title(Some(plain_text("Please enjoy this photo of a kitten")))
            .set_block_id(Some("image4"))
            .set_slack_file(Some(slack_file()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Image::builder()
            .alt_text("An incredibly cute kitten.")
            .title(plain_text("Please enjoy this photo of a kitten"))
            .block_id("image4")
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
    fn it_requires_alt_text_less_than_2000_characters_long() {
        let err = Image::builder()
            .alt_text("a".repeat(2001))
            .slack_file(slack_file())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("alt_text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(2000)));
    }

    #[test]
    fn it_requires_image_url_less_than_3000_characters_long() {
        let err = Image::builder()
            .alt_text("An incredibly cute kitten.")
            .image_url("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("image_url");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(3000)));
    }

    #[test]
    fn it_requires_title_less_than_2000_characters_long() {
        let err = Image::builder()
            .alt_text("An incredibly cute kitten.")
            .image_url("http://placekitten.com/500/500")
            .title(plain_text("a".repeat(2001)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(2000)));
    }

    #[test]
    fn it_requires_block_id_than_255_characters_long() {
        let err = Image::builder()
            .alt_text("An incredibly cute kitten.")
            .image_url("http://placekitten.com/500/500")
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Image");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
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
