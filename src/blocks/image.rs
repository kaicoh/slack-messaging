use super::elements::Text;
use serde::Serialize;

/// [Image block](https://api.slack.com/reference/block-kit/blocks#image)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::Image;
/// use serde_json::json;
///
/// let image = Image::new()
///     .set_block_id("image4")
///     .title("Please enjoy this photo of a kitten")
///     .set_image_url("http://placekitten.com/500/500")
///     .set_alt_text("An incredibly cute kitten.");
///
/// let expected = json!({
///     "type": "image",
///     "block_id": "image4",
///     "title": {
///         "type": "plain_text",
///         "text": "Please enjoy this photo of a kitten",
///         "emoji": true
///     },
///     "image_url": "http://placekitten.com/500/500",
///     "alt_text": "An incredibly cute kitten."
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

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            kind: "image",
            image_url: "".to_string(),
            alt_text: "".to_string(),
            title: None,
            block_id: None,
        }
    }
}

impl Image {
    /// Constructs an Image block.
    ///
    /// ```
    /// use slack_messaging::blocks::Image;
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
    /// use slack_messaging::blocks::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().set_image_url("http://placekitten.com/500/500");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/500/500",
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
    /// use slack_messaging::blocks::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().set_alt_text("An incredibly cute kitten.");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "An incredibly cute kitten."
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

    /// Sets title field.
    ///
    /// ```
    /// use slack_messaging::blocks::Image;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let image = Image::new()
    ///     .set_title(Text::plain("Please enjoy this photo of a kitten"));
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Please enjoy this photo of a kitten",
    ///         "emoji": true
    ///     },
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn set_title(self, text: Text) -> Self {
        Self {
            title: Some(text),
            ..self
        }
    }

    /// Sets title field from string. This is a shorthand for `set_text` method.
    ///
    /// ```
    /// use slack_messaging::blocks::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().title("Please enjoy this photo of a kitten");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Please enjoy this photo of a kitten",
    ///         "emoji": true
    ///     },
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn title<T: Into<String>>(self, title: T) -> Self {
        self.set_title(Text::plain(title))
    }

    /// Sets block_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::Image;
    /// use serde_json::json;
    ///
    /// let image = Image::new().set_block_id("image4");
    ///
    /// let expected = json!({
    ///     "type": "image",
    ///     "block_id": "image4",
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let image_json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(image_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
