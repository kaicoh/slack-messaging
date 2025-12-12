use crate::blocks::rich_text::types::{RichTextStyle, StyleTypeSix};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [user element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextElementUser, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementUser::builder()
///     .user_id("U123ABC456")
///     .style(
///         RichTextStyle::builder()
///             .italic(true)
///             .highlight(true)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "user",
///     "user_id": "U123ABC456",
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
/// let element = RichTextElementUser::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "user")]
pub struct RichTextElementUser {
    #[builder(validate("required"))]
    pub(crate) user_id: Option<String>,

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
        let expected = RichTextElementUser {
            user_id: Some("U123ABC456".into()),
            style: Some(style_six()),
        };

        let val = RichTextElementUser::builder()
            .set_user_id(Some("U123ABC456"))
            .set_style(Some(style_six()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementUser::builder()
            .user_id("U123ABC456")
            .style(style_six())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_user_id_field() {
        let err = RichTextElementUser::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementUser");

        let errors = err.field("user_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
