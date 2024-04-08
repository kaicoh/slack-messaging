use super::composition_objects::SlackFile;
use serde::Serialize;

/// [Image element](https://api.slack.com/reference/block-kit/block-elements#image)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::Image;
/// let image = Image::builder()
///     .image_url("http://placekitten.com/700/500")
///     .alt_text("Multiple cute kittens")
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) alt_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) slack_file: Option<SlackFile>,
}
