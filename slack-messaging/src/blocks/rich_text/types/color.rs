use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [color element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#color-element-type)
/// for rich text.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#color-element-type).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | value | String | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::RichTextElementColor;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementColor::builder()
///     .value("#F405B3")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "color",
///     "value": "#F405B3"
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementColor::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "color")]
pub struct RichTextElementColor {
    #[builder(validate("required"))]
    pub(crate) value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementColor {
            value: Some("#000000".into()),
        };

        let val = RichTextElementColor::builder()
            .set_value(Some("#000000"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementColor::builder()
            .value("#000000")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_value_field() {
        let err = RichTextElementColor::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementColor");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
