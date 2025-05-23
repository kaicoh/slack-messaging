use super::{RichText, RichTextElement};

impl RichText {
    /// Construct a [`RichTextBuilder`].
    pub fn builder() -> RichTextBuilder {
        RichTextBuilder::default()
    }
}

/// Builder for [`RichText`] object.
#[derive(Debug, Default)]
pub struct RichTextBuilder {
    elements: Vec<RichTextElement>,
    block_id: Option<String>,
}

impl RichTextBuilder {
    /// Set elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// # use slack_messaging::blocks::rich_text::elements::{RichTextSection, RichTextElementTypeText};
    /// let rich_text = RichText::builder()
    ///     .set_elements(
    ///         vec![
    ///             RichTextSection::builder()
    ///                 .element(
    ///                     RichTextElementTypeText::builder()
    ///                         .text("Hello")
    ///                         .build()
    ///                 )
    ///                 .build()
    ///                 .into()
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text",
    ///     "elements": [
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Hello"
    ///                 }
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<RichTextElement>) -> Self {
        Self { elements, ..self }
    }

    /// Add RichTextElement object to elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// # use slack_messaging::blocks::rich_text::elements::{RichTextSection, RichTextElementTypeText};
    /// let rich_text = RichText::builder()
    ///     .element(
    ///         RichTextSection::builder()
    ///             .element(
    ///                 RichTextElementTypeText::builder()
    ///                     .text("Hello")
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text",
    ///     "elements": [
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Hello"
    ///                 }
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<RichTextElement>) -> Self {
        let Self { mut elements, .. } = self;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// let rich_text = RichText::builder()
    ///     .set_block_id(Some("block-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text",
    ///     "block_id": "block-0",
    ///     "elements": []
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::RichText;
    /// let rich_text = RichText::builder()
    ///     .block_id("block-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text",
    ///     "block_id": "block-0",
    ///     "elements": []
    /// });
    ///
    /// let json = serde_json::to_value(rich_text).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build a [`RichText`] object.
    pub fn build(self) -> RichText {
        RichText {
            kind: "rich_text",
            elements: self.elements,
            block_id: self.block_id,
        }
    }

    /// Get elements value.
    pub fn get_elements(&self) -> &[RichTextElement] {
        &self.elements
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }
}
