use super::{composition_objects::Text, elements::Image};
use serde::Serialize;

/// [Context block](https://docs.slack.dev/reference/block-kit/blocks/context-block)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the sample context](https://docs.slack.dev/reference/block-kit/blocks/context-block#examples).
///
/// ```
/// # use slack_messaging::mrkdwn;
/// # use slack_messaging::blocks::Context;
/// # use slack_messaging::blocks::elements::Image;
/// let context = Context::builder()
///     .element(
///         Image::builder()
///             .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
///             .alt_text("images")
///             .build()
///     )
///     .element(mrkdwn!("Location: **Dogpatch**"))
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Context {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<ContextElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}

/// Objects that can be an element of the [Context]'s elements field.
#[derive(Debug, Clone, Serialize)]
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
