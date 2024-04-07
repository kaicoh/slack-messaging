use super::SlackFile;
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
    kind: &'static str,

    alt_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    slack_file: Option<SlackFile>,
}

impl Image {
    /// Construct a [`ImageBuilder`].
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }
}

/// Builder for [`Image`] object.
#[derive(Debug, Default)]
pub struct ImageBuilder {
    alt_text: Option<String>,
    image_url: Option<String>,
    slack_file: Option<SlackFile>,
}

impl ImageBuilder {
    /// Set image_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Image;
    /// let image = Image::builder()
    ///     .set_image_url(Some("http://placekitten.com/700/500".into()))
    ///     .alt_text("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/700/500",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_image_url(self, image_url: Option<String>) -> Self {
        Self { image_url, ..self }
    }

    /// Set image_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Image;
    /// let image = Image::builder()
    ///     .image_url("http://placekitten.com/700/500")
    ///     .alt_text("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/700/500",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn image_url(self, image_url: impl Into<String>) -> Self {
        self.set_image_url(Some(image_url.into()))
    }

    /// Set alt_text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Image;
    /// let image = Image::builder()
    ///     .set_alt_text(Some("Multiple cute kittens".into()))
    ///     .image_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "Multiple cute kittens"
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_alt_text(self, alt_text: Option<String>) -> Self {
        Self { alt_text, ..self }
    }

    /// Set alt_text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Image;
    /// let image = Image::builder()
    ///     .alt_text("Multiple cute kittens")
    ///     .image_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "Multiple cute kittens"
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn alt_text(self, alt_text: impl Into<String>) -> Self {
        self.set_alt_text(Some(alt_text.into()))
    }

    /// Set slack_file field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Image, SlackFile};
    /// let image = Image::builder()
    ///     .set_slack_file(
    ///         Some(SlackFile::builder().id("F0123456").build())
    ///     )
    ///     .alt_text("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "alt_text": "",
    ///     "slack_file": {
    ///         "id": "F0123456"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_slack_file(self, slack_file: Option<SlackFile>) -> Self {
        Self { slack_file, ..self }
    }

    /// Set slack_file field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Image, SlackFile};
    /// let image = Image::builder()
    ///     .slack_file(SlackFile::builder().id("F0123456").build())
    ///     .alt_text("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "alt_text": "",
    ///     "slack_file": {
    ///         "id": "F0123456"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn slack_file(self, slack_file: SlackFile) -> Self {
        self.set_slack_file(Some(slack_file))
    }

    /// Build a [`Image`] object. This method will panic if `alt_text` is not set or neither
    /// `image_url` nor `slack_file` are set.
    pub fn build(self) -> Image {
        if self.image_url.is_none() && self.slack_file.is_none() {
            panic!("Either image_url or slack_file must be set to ImageBuilder");
        }

        Image {
            kind: "image",
            alt_text: self.alt_text.expect("alt_text must be set to ImageBuilder"),
            image_url: self.image_url,
            slack_file: self.slack_file,
        }
    }
}
