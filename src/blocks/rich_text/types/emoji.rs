use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [emoji element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#emoji-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::RichTextElementEmoji;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementEmoji::builder()
///     .name("basketball")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "emoji",
///     "name": "basketball"
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementEmoji::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "emoji")]
pub struct RichTextElementEmoji {
    #[builder(validate("required"))]
    pub(crate) name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) unicode: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementEmoji {
            name: Some("snowboarder".into()),
            unicode: Some("foobar".into()),
        };

        let val = RichTextElementEmoji::builder()
            .set_name(Some("snowboarder"))
            .set_unicode(Some("foobar"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementEmoji::builder()
            .name("snowboarder")
            .unicode("foobar")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_name_field() {
        let err = RichTextElementEmoji::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementEmoji");

        let errors = err.field("name");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
