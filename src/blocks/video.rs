use super::elements::Text;
use crate::plain_text;
use serde::Serialize;

/// [Video block](https://api.slack.com/reference/block-kit/blocks#video)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::Video;
/// use serde_json::json;
///
/// let video = Video::new()
///     .title("How to use Slack.")
///     .description("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.")
///     .set_title_url("https://www.youtube.com/watch?v=RRxQQxiM7AA")
///     .set_video_url("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1")
///     .set_thumbnail_url("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg")
///     .set_alt_text("How to use Slack?")
///     .set_author_name("Arcado Buendia")
///     .set_provider_name("YouTube")
///     .set_provider_icon_url("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png");
///
/// let expected = json!({
///     "type": "video",
///     "title": {
///         "type": "plain_text",
///         "text": "How to use Slack.",
///         "emoji": true
///     },
///     "description": {
///         "type": "plain_text",
///         "text": "Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.",
///         "emoji": true
///     },
///     "title_url": "https://www.youtube.com/watch?v=RRxQQxiM7AA",
///     "video_url": "https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1",
///     "thumbnail_url": "https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg",
///     "alt_text": "How to use Slack?",
///     "author_name": "Arcado Buendia",
///     "provider_name": "YouTube",
///     "provider_icon_url": "https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png"
/// });
///
/// let video_json = serde_json::to_value(video).unwrap();
///
/// assert_eq!(video_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Video {
    #[serde(rename = "type")]
    kind: &'static str,

    alt_text: String,

    title: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_url: Option<String>,

    thumbnail_url: String,

    video_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    provider_icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<String>,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            kind: "video",
            alt_text: "".to_string(),
            title: plain_text!(""),
            title_url: None,
            thumbnail_url: "".to_string(),
            video_url: "".to_string(),
            author_name: None,
            block_id: None,
            description: None,
            provider_icon_url: None,
            provider_name: None,
        }
    }
}

impl Video {
    /// Constructs a Video block.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new();
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets alt_text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new().set_alt_text("How to use Slack?");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "How to use Slack?"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_alt_text<T: Into<String>>(self, alt: T) -> Self {
        Self {
            alt_text: alt.into(),
            ..self
        }
    }

    /// Sets title field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_title(Text::plain("How to use Slack."));
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "How to use Slack.",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_title(self, title: Text) -> Self {
        Self { title, ..self }
    }

    /// Sets title_url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_title_url("https://www.youtube.com/watch?v=RRxQQxiM7AA");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "title_url": "https://www.youtube.com/watch?v=RRxQQxiM7AA"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_title_url<T: Into<String>>(self, title_url: T) -> Self {
        Self {
            title_url: Some(title_url.into()),
            ..self
        }
    }

    /// Sets thumbnail_url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_thumbnail_url("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg",
    ///     "alt_text": ""
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_thumbnail_url<T: Into<String>>(self, thumbnail_url: T) -> Self {
        Self {
            thumbnail_url: thumbnail_url.into(),
            ..self
        }
    }

    /// Sets video_url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_video_url("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_video_url<T: Into<String>>(self, video_url: T) -> Self {
        Self {
            video_url: video_url.into(),
            ..self
        }
    }

    /// Sets author_name field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_author_name("Arcado Buendia");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "author_name": "Arcado Buendia"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_author_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            author_name: Some(name.into()),
            ..self
        }
    }

    /// Sets block_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_block_id("slack video");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "block_id": "slack video"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    /// Sets description field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_description(Text::plain("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email."));
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": ""
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_description(self, description: Text) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    /// Sets provider_icon_url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_provider_icon_url("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_icon_url": "https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_provider_icon_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            provider_icon_url: Some(url.into()),
            ..self
        }
    }

    /// Sets provider_name field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Video;
    /// use serde_json::json;
    ///
    /// let video = Video::new()
    ///     .set_provider_name("YouTube");
    ///
    /// let expected = json!({
    ///     "type": "video",
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "video_url": "",
    ///     "thumbnail_url": "",
    ///     "alt_text": "",
    ///     "provider_name": "YouTube"
    /// });
    ///
    /// let video_json = serde_json::to_value(video).unwrap();
    ///
    /// assert_eq!(video_json, expected);
    /// ```
    pub fn set_provider_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            provider_name: Some(name.into()),
            ..self
        }
    }
}
