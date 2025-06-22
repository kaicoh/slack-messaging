use super::{Context, ContextElement};

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
    /// # use slack_messaging::blocks::elements::Image;
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let context = Context::builder()
    ///     .set_elements(
    ///         vec![
    ///             Image::builder()
    ///                 .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///                 .alt_text("images")
    ///                 .build()
    ///                 .into(),
    ///             MrkdwnText::builder()
    ///                 .text("Location: **Dogpatch**")
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
    /// # use slack_messaging::blocks::elements::Image;
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let context = Context::builder()
    ///     .element(
    ///         Image::builder()
    ///             .image_url("https://image.freepik.com/free-photo/red-drawing-pin_1156-445.jpg")
    ///             .alt_text("images")
    ///             .build()
    ///     )
    ///     .element(
    ///         MrkdwnText::builder()
    ///             .text("Location: **Dogpatch**")
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

    /// Get elements value.
    pub fn get_elements(&self) -> &[ContextElement] {
        &self.elements
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }
}
