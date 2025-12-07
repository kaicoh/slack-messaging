use super::composition_objects::SlackFile;
use serde::Serialize;

/// [Image element](https://docs.slack.dev/reference/block-kit/block-elements/image-element)
/// representation.
///
/// The Builder returns [`ImageError`](crate::blocks::elements::builders::ImageError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
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
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "image")]
pub struct Image {
    pub(crate) alt_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) slack_file: Option<SlackFile>,
}
