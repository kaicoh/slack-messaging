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
/// use slack_messaging::blocks::Context;
/// use slack_messaging::blocks::elements::{Image, Text};
/// use serde_json::json;
///
/// let context = Context::new()
///     .push_element(
///         Image::new()
///             .set_image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
///             .set_alt_text("images")
///     )
///     .push_element(
///         Text::mrkdwn("Location: **Dogpatch**")
///     );
///
/// let expected = json!({
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
/// let context_json = serde_json::to_value(context).unwrap();
///
/// assert_eq!(context_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct Context {
    #[serde(rename = "type")]
    kind: &'static str,

    elements: Vec<ContextElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            kind: "context",
            elements: vec![],
            block_id: None,
        }
    }
}

impl Context {
    /// Constructs a Context block.
    ///
    /// ```
    /// use slack_messaging::blocks::Context;
    /// use serde_json::json;
    ///
    /// let context = Context::new();
    ///
    /// let expected = json!({
    ///     "type": "context",
    ///     "elements": []
    /// });
    ///
    /// let context_json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(context_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets elements field directly. The argument is a vector composed from any objects
    /// that can transform into the enum [ContextElement].
    ///
    /// ```
    /// use slack_messaging::blocks::Context;
    /// use slack_messaging::blocks::elements::{Image, Text};
    /// use serde_json::json;
    ///
    /// let context = Context::new()
    ///     .set_elements(
    ///         vec![
    ///             Image::new()
    ///                 .set_image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///                 .set_alt_text("images")
    ///                 .into(),
    ///             Text::mrkdwn("Location: **Dogpatch**").into()
    ///         ]
    ///     );
    ///
    /// let expected = json!({
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
    /// let context_json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(context_json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<ContextElement>) -> Self {
        Self { elements, ..self }
    }

    /// Adds an object to elements field. The argument is an any object
    /// that can transform into the enum [ContextElement].
    ///
    /// ```
    /// use slack_messaging::blocks::Context;
    /// use slack_messaging::blocks::elements::{Image, Text};
    /// use serde_json::json;
    ///
    /// let context = Context::new()
    ///     .push_element(
    ///         Image::new()
    ///             .set_image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///             .set_alt_text("images")
    ///     )
    ///     .push_element(
    ///         Text::mrkdwn("Location: **Dogpatch**")
    ///     );
    ///
    /// let expected = json!({
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
    /// let context_json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(context_json, expected);
    /// ```
    pub fn push_element<T: Into<ContextElement>>(self, element: T) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Sets block_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::Context;
    /// use serde_json::json;
    ///
    /// let context = Context::new().set_block_id("context_block_1");
    ///
    /// let expected = json!({
    ///     "type": "context",
    ///     "elements": [],
    ///     "block_id": "context_block_1"
    /// });
    ///
    /// let context_json = serde_json::to_value(context).unwrap();
    ///
    /// assert_eq!(context_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}

/// Objects that can be an element of the [Context]'s elements field.
#[derive(Debug, Serialize)]
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
