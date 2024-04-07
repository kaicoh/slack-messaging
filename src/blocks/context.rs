use super::elements::{Image, Text};
use serde::Serialize;

/// [Context block](https://api.slack.com/reference/block-kit/blocks#context)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the sample context](https://api.slack.com/reference/block-kit/blocks#context_examples).
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
    kind: &'static str,

    elements: Vec<ContextElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Context {
    /// Construct a [`ContextBuilder`].
    pub fn builder() -> ContextBuilder {
        ContextBuilder::default()
    }
}

/// Builder for [`Context`] object.
#[derive(Debug, Default)]
pub struct ContextBuilder {
    elements: Vec<ContextElement>,
    block_id: Option<String>,
}

impl ContextBuilder {
    /// Set elements field. The argument is a vector composed from any objects
    /// that can transform into the enum [ContextElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Context;
    /// # use slack_messaging::blocks::elements::{Image, Text};
    /// let context = Context::builder()
    ///     .set_elements(
    ///         vec![
    ///             Image::builder()
    ///                 .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///                 .alt_text("images")
    ///                 .build()
    ///                 .into(),
    ///             Text::builder()
    ///                 .mrkdwn("Location: **Dogpatch**")
    ///                 .build()
    ///                 .into(),
    ///         ]
    ///     )
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
    pub fn set_elements(self, elements: Vec<ContextElement>) -> Self {
        Self { elements, ..self }
    }

    /// Add an element to elements field. The argument is an any object
    /// that can transform into the enum [ContextElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Context;
    /// # use slack_messaging::blocks::elements::{Image, Text};
    /// let context = Context::builder()
    ///     .element(
    ///         Image::builder()
    ///             .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///             .alt_text("images")
    ///             .build()
    ///     )
    ///     .element(
    ///         Text::builder()
    ///             .mrkdwn("Location: **Dogpatch**")
    ///             .build()
    ///     )
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
    pub fn element(self, element: impl Into<ContextElement>) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Context;
    /// let context = Context::builder()
    ///     .set_block_id(Some("context_block_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context",
    ///     "elements": [],
    ///     "block_id": "context_block_1"
    /// });
    ///
    /// let json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Context;
    /// let context = Context::builder()
    ///     .block_id("context_block_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context",
    ///     "elements": [],
    ///     "block_id": "context_block_1"
    /// });
    ///
    /// let json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build a [`Context`] object
    pub fn build(self) -> Context {
        Context {
            kind: "context",
            elements: self.elements,
            block_id: self.block_id,
        }
    }
}

/// Objects that can be an element of the [Context]'s elements field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ContextElement {
    /// [Image element](https://api.slack.com/reference/block-kit/block-elements#image)
    /// representation
    Image(Box<Image>),

    /// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
    /// representation
    Text(Box<Text>),
}

impl From<Image> for ContextElement {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
    }
}

impl From<Text> for ContextElement {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}
