use crate::blocks::rich_text::element_types::types::{RichTextStyle, StyleTypeFour};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [text element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#text-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::element_types::{RichTextElementText,
/// types::RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementText::builder()
///     .text("I am a bold rich text block!")
///     .style(
///         RichTextStyle::builder()
///             .bold(true)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "text",
///     "text": "I am a bold rich text block!",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementText::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "text")]
pub struct RichTextElementText {
    #[builder(validate("required"))]
    pub(crate) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<RichTextStyle<StyleTypeFour>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::element_types::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementText {
            text: Some("foobar".into()),
            style: Some(style_four()),
        };

        let val = RichTextElementText::builder()
            .set_text(Some("foobar"))
            .set_style(Some(style_four()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementText::builder()
            .text("foobar")
            .style(style_four())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = RichTextElementText::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementText");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
