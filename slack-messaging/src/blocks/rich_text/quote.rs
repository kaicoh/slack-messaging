use crate::blocks::rich_text::types::RichTextElementType;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Rich text quote element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_quote)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_quote).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | elements | Vec<[RichTextElementType]> | Yes | N/A |
/// | border | i64 | No | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::{RichTextQuote, types::RichTextElementText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let quote = RichTextQuote::builder()
///     .element(
///         RichTextElementText::builder()
///             .text("What we need is good examples in our documentation.")
///             .build()?
///     )
///     .border(0)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text_quote",
///     "elements": [
///         {
///             "type": "text",
///             "text": "What we need is good examples in our documentation."
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(quote).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let quote = RichTextQuote::builder().build();
/// assert!(quote.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text_quote")]
pub struct RichTextQuote {
    #[builder(push_item = "element", validate("required"))]
    pub(crate) elements: Option<Vec<RichTextElementType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) border: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::types::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextQuote {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
            border: Some(3),
        };

        let val = RichTextQuote::builder()
            .set_elements(Some(vec![el_text("foo"), el_emoji("bar")]))
            .set_border(Some(3))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextQuote::builder()
            .elements(vec![el_text("foo"), el_emoji("bar")])
            .border(3)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RichTextQuote {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
            border: None,
        };

        let val = RichTextQuote::builder()
            .element(text("foo"))
            .element(emoji("bar"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requres_elements_field() {
        let err = RichTextQuote::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextQuote");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
