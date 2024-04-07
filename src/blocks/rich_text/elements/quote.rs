use super::types::RichTextElementType;
use serde::Serialize;

/// [Rich text quote element](https://api.slack.com/reference/block-kit/blocks#rich_text_quote)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::RichTextQuote;
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
/// let quote = RichTextQuote::builder()
///     .element(
///         RichTextElementTypeText::builder()
///             .text("What we need is good examples in our documentation.")
///             .build()
///     )
///     .border(0)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_quote",
///     "elements": [
///         {
///             "type": "text",
///             "text": "What we need is good examples in our documentation."
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(quote).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextQuote {
    #[serde(rename = "type")]
    kind: &'static str,

    elements: Vec<RichTextElementType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<i64>,
}

impl RichTextQuote {
    /// Construct a [`RichTextQuoteBuilder`].
    pub fn builder() -> RichTextQuoteBuilder {
        RichTextQuoteBuilder::default()
    }
}

/// Builder for [`RichTextQuote`] object.
#[derive(Debug, Default)]
pub struct RichTextQuoteBuilder {
    elements: Vec<RichTextElementType>,
    border: Option<i64>,
}

impl RichTextQuoteBuilder {
    /// Set elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextQuote;
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
    /// let quote = RichTextQuote::builder()
    ///     .set_elements(
    ///         vec![
    ///             RichTextElementTypeText::builder()
    ///                 .text("What we need is good examples in our documentation.")
    ///                 .build()
    ///                 .into()
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_quote",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "What we need is good examples in our documentation."
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(quote).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<RichTextElementType>) -> Self {
        Self { elements, ..self }
    }

    /// Add RichTextElementType object to elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextQuote;
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
    /// let quote = RichTextQuote::builder()
    ///     .element(
    ///         RichTextElementTypeText::builder()
    ///             .text("What we need is good examples in our documentation.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_quote",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "What we need is good examples in our documentation."
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(quote).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<RichTextElementType>) -> Self {
        let Self { mut elements, .. } = self;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Set border field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextQuote;
    /// let quote = RichTextQuote::builder()
    ///     .set_border(Some(0))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_quote",
    ///     "elements": [],
    ///     "border": 0
    /// });
    ///
    /// let json = serde_json::to_value(quote).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_border(self, border: Option<i64>) -> Self {
        Self { border, ..self }
    }

    /// Set border field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextQuote;
    /// let quote = RichTextQuote::builder()
    ///     .border(0)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_quote",
    ///     "elements": [],
    ///     "border": 0
    /// });
    ///
    /// let json = serde_json::to_value(quote).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn border(self, border: impl Into<i64>) -> Self {
        self.set_border(Some(border.into()))
    }

    /// Build a [`RichTextQuote`] object.
    pub fn build(self) -> RichTextQuote {
        RichTextQuote {
            kind: "rich_text_quote",
            elements: self.elements,
            border: self.border,
        }
    }
}
