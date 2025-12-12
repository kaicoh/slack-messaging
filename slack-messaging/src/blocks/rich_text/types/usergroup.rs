use crate::blocks::rich_text::types::{RichTextStyle, StyleTypeSix};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [usergroup element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-group-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextElementUserGroup, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementUserGroup::builder()
///     .usergroup_id("G123ABC456")
///     .style(
///         RichTextStyle::builder()
///             .italic(true)
///             .highlight(true)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "usergroup",
///     "usergroup_id": "G123ABC456",
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
/// let element = RichTextElementUserGroup::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "usergroup")]
pub struct RichTextElementUserGroup {
    #[builder(validate("required"))]
    pub(crate) usergroup_id: Option<String>,

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
        let expected = RichTextElementUserGroup {
            usergroup_id: Some("G123ABC456".into()),
            style: Some(style_six()),
        };

        let val = RichTextElementUserGroup::builder()
            .set_usergroup_id(Some("G123ABC456"))
            .set_style(Some(style_six()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementUserGroup::builder()
            .usergroup_id("G123ABC456")
            .style(style_six())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_usergroup_id_field() {
        let err = RichTextElementUserGroup::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementUserGroup");

        let errors = err.field("usergroup_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
