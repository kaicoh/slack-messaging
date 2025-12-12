use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Divider block](https://docs.slack.dev/reference/block-kit/blocks/divider-block)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::Divider;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let divider = Divider::builder()
///     .block_id("divider_block")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "divider",
///     "block_id": "divider_block"
/// });
///
/// let json = serde_json::to_value(divider).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "divider")]
pub struct Divider {
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
        let expected = Divider {
            block_id: Some("divider_0".into()),
        };

        let val = Divider::builder()
            .set_block_id(Some("divider_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Divider::builder().block_id("divider_0").build().unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Divider::builder()
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Divider");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }
}
