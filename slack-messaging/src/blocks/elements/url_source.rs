use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [URL source
/// element](https://docs.slack.dev/reference/block-kit/block-elements/url-source-element)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/url-source-element).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | url | String | Yes | N/A |
/// | text | String | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::UrlSource;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let url_source = UrlSource::builder()
///     .url("https://docs.slack.dev/")
///     .text("Slack API docs")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "url",
///     "url": "https://docs.slack.dev/",
///     "text": "Slack API docs"
/// });
///
/// let json = serde_json::to_value(url_source).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let url_source = UrlSource::builder().build();
///
/// assert!(url_source.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "url")]
pub struct UrlSource {
    #[builder(validate("required"))]
    pub(crate) url: Option<String>,

    #[builder(validate("required"))]
    pub(crate) text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = UrlSource {
            url: Some("https://docs.slack.dev/".into()),
            text: Some("Slack API docs".into()),
        };

        let val = UrlSource::builder()
            .set_url(Some("https://docs.slack.dev/"))
            .set_text(Some("Slack API docs"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = UrlSource::builder()
            .url("https://docs.slack.dev/")
            .text("Slack API docs")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_url_field() {
        let err = UrlSource::builder()
            .text("Slack API docs")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "UrlSource");

        let errors = err.field("url");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_field() {
        let err = UrlSource::builder()
            .url("https://docs.slack.dev/")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "UrlSource");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
