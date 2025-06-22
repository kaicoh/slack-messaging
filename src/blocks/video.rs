use super::composition_objects::PlainText;
use serde::Serialize;

/// [Video block](https://docs.slack.dev/reference/block-kit/blocks/video-block)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Video;
/// let video = Video::builder()
///     .title("How to use Slack.")
///     .description("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.")
///     .title_url("https://www.youtube.com/watch?v=RRxQQxiM7AA")
///     .video_url("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1")
///     .thumbnail_url("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg")
///     .alt_text("How to use Slack?")
///     .author_name("Arcado Buendia")
///     .provider_name("YouTube")
///     .provider_icon_url("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "video",
///     "title": {
///         "type": "plain_text",
///         "text": "How to use Slack."
///     },
///     "description": {
///         "type": "plain_text",
///         "text": "Slack is a new way to communicate with your team. It's faster, better organized and more secure than email."
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
/// let json = serde_json::to_value(video).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Video {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) alt_text: String,

    pub(super) title: PlainText,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) title_url: Option<String>,

    pub(super) thumbnail_url: String,

    pub(super) video_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) author_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) description: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) provider_icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) provider_name: Option<String>,
}
