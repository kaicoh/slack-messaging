use super::composition_objects::{SlackFile, Text};
use serde::Serialize;

/// [Image block](https://api.slack.com/reference/block-kit/blocks#image)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Image;
/// let image = Image::builder()
///     .block_id("image4")
///     .title("Please enjoy this photo of a kitten")
///     .image_url("http://placekitten.com/500/500")
///     .alt_text("An incredibly cute kitten.")
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) alt_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) title: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) slack_file: Option<SlackFile>,
}
