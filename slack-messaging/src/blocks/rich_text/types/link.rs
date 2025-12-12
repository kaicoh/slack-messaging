use crate::blocks::rich_text::types::{RichTextStyle, StyleTypeFour};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [link element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#link-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextElementLink, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementLink::builder()
///     .url("https://docs.slack.dev")
///     .r#unsafe(true)
///     .style(
///         RichTextStyle::builder()
///             .bold(true)
///             .code(true)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "link",
///     "url": "https://docs.slack.dev",
///     "unsafe": true,
///     "style": {
///         "bold": true,
///         "code": true
///     }
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementLink::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "link")]
pub struct RichTextElementLink {
    #[builder(validate("required"))]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) r#unsafe: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<RichTextStyle<StyleTypeFour>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::types::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementLink {
            url: Some("https://docs.slack.dev".into()),
            text: Some("foobar".into()),
            r#unsafe: Some(true),
            style: Some(style_four()),
        };

        let val = RichTextElementLink::builder()
            .set_url(Some("https://docs.slack.dev"))
            .set_text(Some("foobar"))
            .set_unsafe(Some(true))
            .set_style(Some(style_four()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementLink::builder()
            .url("https://docs.slack.dev")
            .text("foobar")
            .r#unsafe(true)
            .style(style_four())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_url_field() {
        let err = RichTextElementLink::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementLink");

        let errors = err.field("url");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
