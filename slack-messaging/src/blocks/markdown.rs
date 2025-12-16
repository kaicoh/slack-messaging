use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Markdown block](https://docs.slack.dev/reference/block-kit/blocks/markdown-block)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/markdown-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | String | Yes | Maximum 12000 characters |
/// | block_id | String | No | Maximum 255 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::Markdown;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let markdown = Markdown::builder()
///     .block_id("markdown-0")
///     .text("**Lots of information here!!**")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "markdown",
///     "block_id": "markdown-0",
///     "text": "**Lots of information here!!**"
/// });
///
/// let json = serde_json::to_value(markdown).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "markdown")]
pub struct Markdown {
    #[builder(validate("required", "text::max_12000"))]
    pub(crate) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Markdown {
            text: Some("**Lots of information here!!**".into()),
            block_id: Some("markdown_0".into()),
        };

        let val = Markdown::builder()
            .set_text(Some("**Lots of information here!!**"))
            .set_block_id(Some("markdown_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Markdown::builder()
            .text("**Lots of information here!!**")
            .block_id("markdown_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = Markdown::builder().build().unwrap_err();
        assert_eq!(err.object(), "Markdown");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_less_than_12000_characters_long() {
        let err = Markdown::builder()
            .text("a".repeat(12001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Markdown");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(12000)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Markdown::builder()
            .text("foo")
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Markdown");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
