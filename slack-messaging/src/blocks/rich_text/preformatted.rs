use crate::blocks::rich_text::types::RichTextElementType;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Rich text preformatted element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_preformatted)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::{RichTextPreformatted, types::RichTextElementText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let preformatted = RichTextPreformatted::builder()
///     .element(
///         RichTextElementText::builder()
///             .text("{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}")
///             .build()?
///     )
///     .border(0)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text_preformatted",
///     "elements": [
///         {
///             "type": "text",
///             "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(preformatted).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let preformatted = RichTextPreformatted::builder().build();
/// assert!(preformatted.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text_preformatted")]
pub struct RichTextPreformatted {
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
        let expected = RichTextPreformatted {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
            border: Some(3),
        };

        let val = RichTextPreformatted::builder()
            .set_elements(Some(vec![el_text("foo"), el_emoji("bar")]))
            .set_border(Some(3))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextPreformatted::builder()
            .elements(vec![el_text("foo"), el_emoji("bar")])
            .border(3)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RichTextPreformatted {
            elements: Some(vec![el_text("foo"), el_emoji("bar")]),
            border: None,
        };

        let val = RichTextPreformatted::builder()
            .element(text("foo"))
            .element(emoji("bar"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requres_elements_field() {
        let err = RichTextPreformatted::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextPreformatted");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
