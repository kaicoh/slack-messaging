use serde::Serialize;

/// [Image element](https://api.slack.com/reference/block-kit/block-elements#image)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::Image;
/// use serde_json::json;
///
/// let image = Image::new()
///     .set_image_url("http://placekitten.com/700/500")
///     .set_alt_text("Multiple cute kittens");
///
/// let expected = json!({
///     "type": "image",
///     "image_url": "http://placekitten.com/700/500",
///     "alt_text": "Multiple cute kittens"
/// });
///
/// let image_json = serde_json::to_value(image).unwrap();
///
/// assert_eq!(image_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    kind: &'static str,

    image_url: String,

    alt_text: String,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            kind: "image",
            image_url: "".into(),
            alt_text: "".into(),
        }
    }
}

impl Image {
    /// Constructs a Image element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new();
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets image_url field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().set_image_url("http://placekitten.com/700/500");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/700/500",
    ///     "alt_text": ""
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn set_image_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            image_url: url.into(),
            ..self
        }
    }

    /// Sets alt_text field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().set_alt_text("Multiple cute kittens");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "Multiple cute kittens"
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn set_alt_text<T: Into<String>>(self, alt: T) -> Self {
        Self {
            alt_text: alt.into(),
            ..self
        }
    }
}
