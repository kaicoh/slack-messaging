use crate::validators::*;

use derive_macro::Builder;
use paste::paste;
use serde::Serialize;

/// Builder objects for [rich text sub-elements](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#usage-info).
pub mod builders;

/// Objects representing [rich text element types](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#element-types).
pub mod types;

mod list;
mod preformatted;
mod quote;
mod section;

pub use list::{ListStyle, RichTextList};
pub use preformatted::RichTextPreformatted;
pub use quote::RichTextQuote;
pub use section::RichTextSection;

/// [Rich text sub elements](https://api.slack.com/reference/block-kit/blocks#element-types)
/// representation.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RichTextSubElement {
    /// [Rich text section element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_section)
    Section(Box<RichTextSection>),

    /// [Rich text list element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_list)
    List(Box<RichTextList>),

    /// [Rich text preformatted element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_preformatted)
    Preformatted(Box<RichTextPreformatted>),

    /// [Rich text quote element](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#rich_text_quote)
    Quote(Box<RichTextQuote>),
}

macro_rules! impl_sub_element {
    ($($var:tt,)*) => {
        paste! {
            $(
                impl From<[<RichText $var>]> for RichTextSubElement {
                    fn from(value: [<RichText $var>]) -> Self {
                        Self::$var(Box::new(value))
                    }
                }
            )*
        }
    };
}

impl_sub_element! {
    Section,
    List,
    Preformatted,
    Quote,
}

// [Rich text block](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block) representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::RichText;
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::{RichTextElementText, RichTextStyle};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let rich_text = RichText::builder()
///     .block_id("rich-text-block-0")
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementText::builder()
///                     .text("Hello there, ")
///                     .build()?
///             )
///             .element(
///                 RichTextElementText::builder()
///                     .text("I am a bold rich text block!")
///                     .style(
///                         RichTextStyle::builder()
///                             .bold(true)
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text",
///     "block_id": "rich-text-block-0",
///     "elements": [
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Hello there, "
///                 },
///                 {
///                     "type": "text",
///                     "text": "I am a bold rich text block!",
///                     "style": {
///                         "bold": true
///                     }
///                 }
///             ]
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(rich_text).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let rich_text = RichText::builder().build();
/// assert!(rich_text.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text")]
pub struct RichText {
    #[builder(push_item = "element", validate("required"))]
    pub(crate) elements: Option<Vec<RichTextSubElement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use super::types::test_helpers::*;
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichText {
            block_id: Some("rich_text_0".into()),
            elements: Some(vec![section(vec![el_text("foo"), el_emoji("var")]).into()]),
        };

        let val = RichText::builder()
            .set_block_id(Some("rich_text_0"))
            .set_elements(Some(vec![
                section(vec![el_text("foo"), el_emoji("var")]).into(),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichText::builder()
            .block_id("rich_text_0")
            .elements(vec![section(vec![el_text("foo"), el_emoji("var")]).into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RichText {
            block_id: None,
            elements: Some(vec![
                section(vec![el_text("foo"), el_emoji("var")]).into(),
                section(vec![el_text("baz")]).into(),
            ]),
        };

        let val = RichText::builder()
            .element(section(vec![el_text("foo"), el_emoji("var")]))
            .element(section(vec![el_text("baz")]))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_elements_field() {
        let err = RichText::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichText");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = RichText::builder()
            .block_id("a".repeat(256))
            .element(section(vec![el_text("baz")]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichText");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }
}

#[cfg(test)]
pub mod test_helpers {
    use super::types::RichTextElementType;
    use super::*;

    pub fn section(elements: Vec<RichTextElementType>) -> RichTextSection {
        RichTextSection {
            elements: Some(elements),
        }
    }
}
