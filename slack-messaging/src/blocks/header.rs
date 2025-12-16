use crate::composition_objects::{Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Header block](https://docs.slack.dev/reference/block-kit/blocks/header-block)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/header-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | [Text<Plain>] | Yes | Maximum 150 characters |
/// | block_id | String | No | Maximum 255 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Header;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let header = Header::builder()
///     .text(plain_text!("Budget Performance")?)
///     .block_id("header_1")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "header",
///     "block_id": "header_1",
///     "text": {
///         "type": "plain_text",
///         "text": "Budget Performance"
///     }
/// });
///
/// let json = serde_json::to_value(header).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "header")]
pub struct Header {
    #[builder(validate("required", "text_object::max_150"))]
    pub(crate) text: Option<Text<Plain>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Header {
            text: Some(plain_text("foo")),
            block_id: Some("header_0".into()),
        };

        let val = Header::builder()
            .set_text(Some(plain_text("foo")))
            .set_block_id(Some("header_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Header::builder()
            .text(plain_text("foo"))
            .block_id("header_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = Header::builder().build().unwrap_err();
        assert_eq!(err.object(), "Header");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_less_than_150_characters_long() {
        let err = Header::builder()
            .text(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Header");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Header::builder()
            .text(plain_text("foo"))
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Header");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
