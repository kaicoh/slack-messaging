use crate::blocks::elements::Image;
use crate::composition_objects::Text;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Context block](https://docs.slack.dev/reference/block-kit/blocks/context-block)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the sample context](https://docs.slack.dev/reference/block-kit/blocks/context-block#examples).
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::Context;
/// use slack_messaging::blocks::elements::Image;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let context = Context::builder()
///     .element(
///         Image::builder()
///             .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
///             .alt_text("images")
///             .build()?
///     )
///     .element(mrkdwn!("Location: **Dogpatch**")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "context",
///     "elements": [
///         {
///             "type": "image",
///             "image_url": "https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg",
///             "alt_text": "images"
///         },
///         {
///             "type": "mrkdwn",
///             "text": "Location: **Dogpatch**"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(context).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "context")]
pub struct Context {
    #[builder(push_item = "element", validate("required", "list::max_item_10"))]
    pub(crate) elements: Option<Vec<ContextElement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

/// Objects that can be an element of the [Context]'s elements field.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ContextElement {
    /// [Image element](https://docs.slack.dev/reference/block-kit/block-elements/image-element)
    /// representation
    Image(Box<Image>),

    /// [Text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
    /// representation
    Text(Box<Text>),
}

impl From<Image> for ContextElement {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
    }
}

impl<T> From<T> for ContextElement
where
    Text: From<T>,
{
    fn from(value: T) -> Self {
        Self::Text(Box::new(Text::from(value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Context {
            block_id: Some("context_0".into()),
            elements: Some(vec![mrkdwn_text("foo").into()]),
        };

        let val = Context::builder()
            .set_block_id(Some("context_0"))
            .set_elements(Some(vec![mrkdwn_text("foo").into()]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Context::builder()
            .block_id("context_0")
            .elements(vec![mrkdwn_text("foo").into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Context {
            block_id: None,
            elements: Some(vec![mrkdwn_text("foo").into()]),
        };

        let val = Context::builder()
            .element(mrkdwn_text("foo"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_elements_field() {
        let err = Context::builder().build().unwrap_err();
        assert_eq!(err.object(), "Context");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_elements_list_size_less_than_10() {
        let elements: Vec<ContextElement> = (0..11).map(|_| mrkdwn_text("foo").into()).collect();
        let err = Context::builder().elements(elements).build().unwrap_err();
        assert_eq!(err.object(), "Context");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(10)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Context::builder()
            .block_id("a".repeat(256))
            .element(mrkdwn_text("foo"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Context");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }
}
