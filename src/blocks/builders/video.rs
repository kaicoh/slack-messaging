use super::{composition_objects::Text, Video};

impl Video {
    /// Construct a [`VideoBuilder`].
    pub fn builder() -> VideoBuilder {
        VideoBuilder::default()
    }
}

/// Builder for [`Video`] object.
#[derive(Debug, Default)]
pub struct VideoBuilder {
    alt_text: Option<String>,
    title: Option<Text>,
    title_url: Option<String>,
    thumbnail_url: Option<String>,
    video_url: Option<String>,
    author_name: Option<String>,
    block_id: Option<String>,
    description: Option<Text>,
    provider_icon_url: Option<String>,
    provider_name: Option<String>,
}

impl VideoBuilder {
    /// Set alt_text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_alt_text(Some("How to use Slack?".into()))
    ///     .title("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "How to use Slack?"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_alt_text(self, alt_text: Option<String>) -> Self {
        Self { alt_text, ..self }
    }

    /// Set alt_text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .alt_text("How to use Slack?")
    ///     .title("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "How to use Slack?"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn alt_text(self, alt_text: impl Into<String>) -> Self {
        self.set_alt_text(Some(alt_text.into()))
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::{blocks::Video, plain_text};
    /// let video = Video::builder()
    ///     .set_title(Some(plain_text!("How to use Slack.")))
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "How to use Slack."
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_title(self, title: Option<Text>) -> Self {
        Self { title, ..self }
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .title("How to use Slack.")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "How to use Slack."
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn title(self, title: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(title).build();
        self.set_title(Some(text))
    }

    /// Set title_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_title_url(Some("https://www.youtube.com/watch?v=RRxQQxiM7AA".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "title_url": "https://www.youtube.com/watch?v=RRxQQxiM7AA"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_title_url(self, title_url: Option<String>) -> Self {
        Self { title_url, ..self }
    }

    /// Set title_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .title_url("https://www.youtube.com/watch?v=RRxQQxiM7AA")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "title_url": "https://www.youtube.com/watch?v=RRxQQxiM7AA"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn title_url(self, title_url: impl Into<String>) -> Self {
        self.set_title_url(Some(title_url.into()))
    }

    /// Set thumbnail_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_thumbnail_url(Some("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_thumbnail_url(self, thumbnail_url: Option<String>) -> Self {
        Self {
            thumbnail_url,
            ..self
        }
    }

    /// Set thumbnail_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .thumbnail_url("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn thumbnail_url(self, thumbnail_url: impl Into<String>) -> Self {
        self.set_thumbnail_url(Some(thumbnail_url.into()))
    }

    /// Set video_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_video_url(Some("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_video_url(self, video_url: Option<String>) -> Self {
        Self { video_url, ..self }
    }

    /// Set video_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .video_url("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1")
    ///     .title("")
    ///     .alt_text("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn video_url(self, video_url: impl Into<String>) -> Self {
        self.set_video_url(Some(video_url.into()))
    }

    /// Set author_name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_author_name(Some("Arcado Buendia".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "author_name": "Arcado Buendia"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_author_name(self, author_name: Option<String>) -> Self {
        Self {
            author_name,
            ..self
        }
    }

    /// Set author_name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .author_name("Arcado Buendia")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "author_name": "Arcado Buendia"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn author_name(self, author_name: impl Into<String>) -> Self {
        self.set_author_name(Some(author_name.into()))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_block_id(Some("slack video".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "block_id": "slack video"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .block_id("slack video")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "block_id": "slack video"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set description field.
    ///
    /// ```
    /// use slack_messaging::{blocks::Video, plain_text};
    /// let video = Video::builder()
    ///     .set_description(Some(plain_text!("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.")))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "Slack is a new way to communicate with your team. It's faster, better organized and more secure than email."
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_description(self, description: Option<Text>) -> Self {
        Self {
            description,
            ..self
        }
    }

    /// Set description field.
    ///
    /// ```
    /// use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .description("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "Slack is a new way to communicate with your team. It's faster, better organized and more secure than email."
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn description(self, description: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(description).build();
        self.set_description(Some(text))
    }

    /// Set provider_icon_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_provider_icon_url(Some("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_icon_url": "https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_provider_icon_url(self, url: Option<String>) -> Self {
        Self {
            provider_icon_url: url,
            ..self
        }
    }

    /// Set provider_icon_url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .provider_icon_url("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_icon_url": "https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn provider_icon_url(self, url: impl Into<String>) -> Self {
        self.set_provider_icon_url(Some(url.into()))
    }

    /// Set provider_name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .set_provider_name(Some("YouTube".into()))
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_name": "YouTube"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_provider_name(self, provider_name: Option<String>) -> Self {
        Self {
            provider_name,
            ..self
        }
    }

    /// Set provider_name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Video;
    /// let video = Video::builder()
    ///     .provider_name("YouTube")
    ///     .title("")
    ///     .alt_text("")
    ///     .video_url("")
    ///     .thumbnail_url("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_name": "YouTube"
    /// });
    ///
    /// let json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn provider_name(self, provider_name: impl Into<String>) -> Self {
        self.set_provider_name(Some(provider_name.into()))
    }

    /// Build a [`Video`] object. This method will panic if any of the alt_text, title,
    /// thumbnail_url and video_url are not set.
    pub fn build(self) -> Video {
        Video {
            kind: "video",
            alt_text: self.alt_text.expect("alt_text must be set to VideoBuilder"),
            title: self.title.expect("title must be set to VideoBuilder"),
            title_url: self.title_url,
            thumbnail_url: self
                .thumbnail_url
                .expect("thumbnail_url must be set to VideoBuilder"),
            video_url: self
                .video_url
                .expect("video_url must be set to VideoBuilder"),
            author_name: self.author_name,
            block_id: self.block_id,
            description: self.description,
            provider_icon_url: self.provider_icon_url,
            provider_name: self.provider_name,
        }
    }

    /// Get alt_text value.
    pub fn get_alt_text(&self) -> &Option<String> {
        &self.alt_text
    }

    /// Get title value.
    pub fn get_title(&self) -> &Option<Text> {
        &self.title
    }

    /// Get title_url value.
    pub fn get_title_url(&self) -> &Option<String> {
        &self.title_url
    }

    /// Get thumbnail_url value.
    pub fn get_thumbnail_url(&self) -> &Option<String> {
        &self.thumbnail_url
    }

    /// Get video_url value.
    pub fn get_video_url(&self) -> &Option<String> {
        &self.video_url
    }

    /// Get author_name value.
    pub fn get_author_name(&self) -> &Option<String> {
        &self.author_name
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get description value.
    pub fn get_description(&self) -> &Option<Text> {
        &self.description
    }

    /// Get provider_icon_url value.
    pub fn get_provider_icon_url(&self) -> &Option<String> {
        &self.provider_icon_url
    }

    /// Get provider_name value.
    pub fn get_provider_name(&self) -> &Option<String> {
        &self.provider_name
    }
}
