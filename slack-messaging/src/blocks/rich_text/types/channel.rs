use crate::blocks::rich_text::types::{RichTextStyle, StyleTypeSix};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [channel element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#channel-element-type)
/// for rich text.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#channel-element-type).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | channel_id | String | Yes | N/A |
/// | style | [RichTextStyle<StyleTypeSix>] | No | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextElementChannel, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementChannel::builder()
///     .channel_id("C123ABC456")
///     .style(
///         RichTextStyle::builder()
///             .italic(true)
///             .highlight(true)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "channel",
///     "channel_id": "C123ABC456",
///     "style": {
///         "italic": true,
///         "highlight": true
///     }
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementChannel::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "channel")]
pub struct RichTextElementChannel {
    #[builder(validate("required"))]
    pub(crate) channel_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<RichTextStyle<StyleTypeSix>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::types::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementChannel {
            channel_id: Some("C123ABC456".into()),
            style: Some(style_six()),
        };

        let val = RichTextElementChannel::builder()
            .set_channel_id(Some("C123ABC456"))
            .set_style(Some(style_six()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementChannel::builder()
            .channel_id("C123ABC456")
            .style(style_six())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_channel_id_field() {
        let err = RichTextElementChannel::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementChannel");

        let errors = err.field("channel_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
