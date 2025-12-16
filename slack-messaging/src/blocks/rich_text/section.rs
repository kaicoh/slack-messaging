use crate::blocks::rich_text::types::RichTextElementType;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Rich text section element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_section)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_section).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | elements | Vec<[RichTextElementType]> | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::{RichTextElementText, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let section = RichTextSection::builder()
///     .element(
///         RichTextElementText::builder()
///             .text("Hello there, ")
///             .build()?
///     )
///     .element(
///         RichTextElementText::builder()
///             .text("I am a bold rich text block!")
///             .style(
///                 RichTextStyle::builder()
///                     .bold(true)
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text_section",
///     "elements": [
///         {
///             "type": "text",
///             "text": "Hello there, "
///         },
///         {
///             "type": "text",
///             "text": "I am a bold rich text block!",
///             "style": {
///                 "bold": true
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(section).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextSection::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text_section")]
pub struct RichTextSection {
    #[builder(push_item = "element", validate("required"))]
    pub(crate) elements: Option<Vec<RichTextElementType>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::types::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextSection {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
        };

        let val = RichTextSection::builder()
            .set_elements(Some(vec![el_text("foo"), el_emoji("bar")]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextSection::builder()
            .elements(vec![el_text("foo"), el_emoji("bar")])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RichTextSection {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
        };

        let val = RichTextSection::builder()
            .element(text("foo"))
            .element(emoji("bar"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requres_elements_field() {
        let err = RichTextSection::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextSection");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
