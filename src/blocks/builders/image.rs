use super::{
    Image,
    composition_objects::{PlainText, SlackFile},
};

impl Image {
    /// Construct an [`ImageBuilder`].
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }
}

/// Builder for [`Image`] object.
#[derive(Debug, Default)]
pub struct ImageBuilder {
    alt_text: Option<String>,
    image_url: Option<String>,
    title: Option<PlainText>,
    block_id: Option<String>,
    slack_file: Option<SlackFile>,
}

impl ImageBuilder {
    /// Set image_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .alt_text("")
    ///     .set_image_url(Some("http://placekitten.com/500/500".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/500/500",
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
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .alt_text("")
    ///     .image_url("http://placekitten.com/500/500")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "http://placekitten.com/500/500",
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
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .set_alt_text(Some("An incredibly cute kitten.".into()))
    ///     .image_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "An incredibly cute kitten."
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
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .alt_text("An incredibly cute kitten.")
    ///     .image_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "image_url": "",
    ///     "alt_text": "An incredibly cute kitten."
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn alt_text(self, alt_text: impl Into<String>) -> Self {
        self.set_alt_text(Some(alt_text.into()))
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let image = Image::builder()
    ///     .image_url("")
    ///     .alt_text("")
    ///     .set_title(
    ///         Some(PlainText::builder()
    ///             .text("Please enjoy this photo of a kitten")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Please enjoy this photo of a kitten"
    ///     },
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_title(self, title: Option<PlainText>) -> Self {
        Self { title, ..self }
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .image_url("")
    ///     .alt_text("")
    ///     .title("Please enjoy this photo of a kitten")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Please enjoy this photo of a kitten"
    ///     },
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn title(self, title: impl Into<String>) -> Self {
        let text = PlainText::builder().text(title).build();
        self.set_title(Some(text))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .image_url("")
    ///     .alt_text("")
    ///     .set_block_id(Some("image4".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "block_id": "image4",
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// let image = Image::builder()
    ///     .image_url("")
    ///     .alt_text("")
    ///     .block_id("image4")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "image",
    ///     "block_id": "image4",
    ///     "image_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(image).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set slack_file field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Image;
    /// # use slack_messaging::composition_objects::SlackFile;
    /// let image = Image::builder()
    ///     .alt_text("")
    ///     .set_slack_file(
    ///         Some(SlackFile::builder()
    ///             .id("F0123456")
    ///             .build())
    ///     )
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
    /// # use slack_messaging::blocks::Image;
    /// # use slack_messaging::composition_objects::SlackFile;
    /// let image = Image::builder()
    ///     .alt_text("")
    ///     .slack_file(
    ///         SlackFile::builder()
    ///             .id("F0123456")
    ///             .build()
    ///     )
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

    /// Build an [`Image`] object. This method will panic either `image_url` or `slack_file` is not set or `alt_text` is not set.
    pub fn build(self) -> Image {
        if self.image_url.is_none() && self.slack_file.is_none() {
            panic!("Either image_url or slack_file must be set to ImageBuilder");
        }

        Image {
            kind: "image",
            alt_text: self.alt_text.expect("alt_text must be set to ImageBuilder"),
            image_url: self.image_url,
            title: self.title,
            block_id: self.block_id,
            slack_file: self.slack_file,
        }
    }

    /// Get alt_text value.
    pub fn get_alt_text(&self) -> &Option<String> {
        &self.alt_text
    }

    /// Get image_url value.
    pub fn get_image_url(&self) -> &Option<String> {
        &self.image_url
    }

    /// Get title value.
    pub fn get_title(&self) -> &Option<PlainText> {
        &self.title
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get slack_file value.
    pub fn get_slack_file(&self) -> &Option<SlackFile> {
        &self.slack_file
    }
}
