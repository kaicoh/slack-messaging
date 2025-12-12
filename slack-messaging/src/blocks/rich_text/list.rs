use super::RichTextSection;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Rich text list element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_list)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::{RichTextList, ListStyle, RichTextSection};
/// use slack_messaging::blocks::rich_text::types::RichTextElementText;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let list = RichTextList::builder()
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementText::builder()
///                     .text("Huddles")
///                     .build()?
///             )
///             .build()?
///     )
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementText::builder()
///                     .text("Canvas")
///                     .build()?
///             )
///             .build()?
///     )
///     .style(ListStyle::Bullet)
///     .indent(0)
///     .border(1)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text_list",
///     "elements": [
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Huddles",
///                 }
///             ]
///         },
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Canvas",
///                 }
///             ]
///         }
///     ],
///     "style": "bullet",
///     "indent": 0,
///     "border": 1
/// });
///
/// let json = serde_json::to_value(list).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let list = RichTextList::builder().build();
/// assert!(list.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text_list")]
pub struct RichTextList {
    #[builder(validate("required"))]
    pub(crate) style: Option<ListStyle>,

    #[builder(push_item = "element", validate("required"))]
    pub(crate) elements: Option<Vec<RichTextSection>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) indent: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) border: Option<i64>,
}

/// List style for [`RichTextList`].
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListStyle {
    Bullet,
    Ordered,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::{test_helpers::*, types::test_helpers::*};
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextList {
            style: Some(ListStyle::Bullet),
            elements: Some(vec![
                section(vec![el_text("foo")]),
                section(vec![el_text("bar")]),
            ]),
            indent: Some(1),
            offset: Some(2),
            border: Some(3),
        };

        let val = RichTextList::builder()
            .set_style(Some(ListStyle::Bullet))
            .set_elements(Some(vec![
                section(vec![el_text("foo")]),
                section(vec![el_text("bar")]),
            ]))
            .set_indent(Some(1))
            .set_offset(Some(2))
            .set_border(Some(3))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextList::builder()
            .style(ListStyle::Bullet)
            .elements(vec![
                section(vec![el_text("foo")]),
                section(vec![el_text("bar")]),
            ])
            .indent(1)
            .offset(2)
            .border(3)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RichTextList {
            style: Some(ListStyle::Ordered),
            elements: Some(vec![
                section(vec![el_text("foo")]),
                section(vec![el_text("bar")]),
            ]),
            indent: None,
            offset: None,
            border: None,
        };

        let val = RichTextList::builder()
            .style(ListStyle::Ordered)
            .element(section(vec![el_text("foo")]))
            .element(section(vec![el_text("bar")]))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_style_field() {
        let err = RichTextList::builder()
            .element(section(vec![el_text("foo")]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextList");

        let errors = err.field("style");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_elements_field() {
        let err = RichTextList::builder()
            .style(ListStyle::Ordered)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextList");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
