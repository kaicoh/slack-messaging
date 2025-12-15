use crate::composition_objects::PlainText;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Video block](https://docs.slack.dev/reference/block-kit/blocks/video-block)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Video;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let video = Video::builder()
///     .title(plain_text!("How to use Slack.")?)
///     .description(plain_text!("Slack is a new way to communicate with your team. It's faster, better organized and more secure than email.")?)
///     .title_url("https://www.youtube.com/watch?v=RRxQQxiM7AA")
///     .video_url("https://www.youtube.com/embed/RRxQQxiM7AA?feature=oembed&autoplay=1")
///     .thumbnail_url("https://i.ytimg.com/vi/RRxQQxiM7AA/hqdefault.jpg")
///     .alt_text("How to use Slack?")
///     .author_name("Arcado Buendia")
///     .provider_name("YouTube")
///     .provider_icon_url("https://a.slack-edge.com/80588/img/unfurl_icons/youtube.png")
///     .build()?;
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
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "video")]
pub struct Video {
    #[builder(validate("required"))]
    pub(crate) alt_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_50"))]
    pub(crate) author_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_200"))]
    pub(crate) description: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) provider_icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) provider_name: Option<String>,

    #[builder(validate("required", "text_object::max_200"))]
    pub(crate) title: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) title_url: Option<String>,

    #[builder(validate("required"))]
    pub(crate) thumbnail_url: Option<String>,

    #[builder(validate("required"))]
    pub(crate) video_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Video {
            alt_text: Some("Use the Events API to create a dynamic App Home".into()),
            author_name: Some("foo".into()),
            block_id: Some("video_0".into()),
            description: Some(plain_text("Slack sure is nifty!")),
            provider_icon_url: Some("bar".into()),
            provider_name: Some("baz".into()),
            title: Some(plain_text(
                "Use the Events API to create a dynamic App Home",
            )),
            title_url: Some("https://www.youtube.com/watch?v=8876OZV_Yy0".into()),
            thumbnail_url: Some("https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg".into()),
            video_url: Some(
                "https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1".into(),
            ),
        };

        let val = Video::builder()
            .set_alt_text(Some("Use the Events API to create a dynamic App Home"))
            .set_author_name(Some("foo"))
            .set_block_id(Some("video_0"))
            .set_description(Some(plain_text("Slack sure is nifty!")))
            .set_provider_icon_url(Some("bar"))
            .set_provider_name(Some("baz"))
            .set_title(Some(plain_text(
                "Use the Events API to create a dynamic App Home",
            )))
            .set_title_url(Some("https://www.youtube.com/watch?v=8876OZV_Yy0"))
            .set_thumbnail_url(Some("https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg"))
            .set_video_url(Some(
                "https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1",
            ))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Video::builder()
            .alt_text("Use the Events API to create a dynamic App Home")
            .author_name("foo")
            .block_id("video_0")
            .description(plain_text("Slack sure is nifty!"))
            .provider_icon_url("bar")
            .provider_name("baz")
            .title(plain_text(
                "Use the Events API to create a dynamic App Home",
            ))
            .title_url("https://www.youtube.com/watch?v=8876OZV_Yy0")
            .thumbnail_url("https://i.ytimg.com/vi/8876OZV_Yy0/hqdefault.jpg")
            .video_url("https://www.youtube.com/embed/8876OZV_Yy0?feature=oembed&autoplay=1")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_alt_text_field() {
        let err = Video::builder()
            .title(plain_text("foo"))
            .thumbnail_url("bar")
            .video_url("baz")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("alt_text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_author_name_less_than_50_characters_long() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("foo"))
            .thumbnail_url("bar")
            .video_url("baz")
            .author_name("a".repeat(51))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("author_name");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(50)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("foo"))
            .thumbnail_url("bar")
            .video_url("baz")
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_description_less_than_200_characters_long() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("foo"))
            .thumbnail_url("bar")
            .video_url("baz")
            .description(plain_text("a".repeat(201)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("description");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(200)));
    }

    #[test]
    fn it_requires_title_field() {
        let err = Video::builder()
            .alt_text("foobar")
            .thumbnail_url("bar")
            .video_url("baz")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_title_less_than_200_characters_long() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("a".repeat(201)))
            .thumbnail_url("bar")
            .video_url("baz")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(200)));
    }

    #[test]
    fn it_requires_thumbnail_url_field() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("foo"))
            .video_url("baz")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("thumbnail_url");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_video_url_field() {
        let err = Video::builder()
            .alt_text("foobar")
            .title(plain_text("foo"))
            .thumbnail_url("bar")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Video");

        let errors = err.field("video_url");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
