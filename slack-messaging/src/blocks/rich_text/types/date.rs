use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [date element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#date-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::RichTextElementDate;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementDate::builder()
///     .timestamp(1720710212)
///     .format("{date_num} at {time}")
///     .fallback("timey")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "date",
///     "timestamp": 1720710212,
///     "format": "{date_num} at {time}",
///     "fallback": "timey"
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementDate::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "date")]
pub struct RichTextElementDate {
    #[builder(validate("required"))]
    pub(crate) timestamp: Option<i64>,

    #[builder(validate("required"))]
    pub(crate) format: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fallback: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementDate {
            timestamp: Some(1720710212),
            format: Some("{date_num} at {time}".into()),
            url: Some("https://google.com".into()),
            fallback: Some("timey".into()),
        };

        let val = RichTextElementDate::builder()
            .set_timestamp(Some(1720710212))
            .set_format(Some("{date_num} at {time}"))
            .set_url(Some("https://google.com"))
            .set_fallback(Some("timey"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementDate::builder()
            .timestamp(1720710212)
            .format("{date_num} at {time}")
            .url("https://google.com")
            .fallback("timey")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_timestamp_field() {
        let err = RichTextElementDate::builder()
            .format("{date_num} at {time}")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextElementDate");

        let errors = err.field("timestamp");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_format_field() {
        let err = RichTextElementDate::builder()
            .timestamp(1720710212)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextElementDate");

        let errors = err.field("format");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
