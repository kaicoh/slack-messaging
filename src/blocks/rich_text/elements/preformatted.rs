use super::types::RichTextElementType;
use serde::Serialize;

/// [Rich text preformatted element](https://api.slack.com/reference/block-kit/blocks#rich_text_preformatted)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::RichTextPreformatted;
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
/// let preformatted = RichTextPreformatted::builder()
///     .element(
///         RichTextElementTypeText::builder()
///             .text("{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}")
///             .build()
///     )
///     .border(0)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_preformatted",
///     "elements": [
///         {
///             "type": "text",
///             "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
///         }
///     ],
///     "border": 0
/// });
///
/// let json = serde_json::to_value(preformatted).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextPreformatted {
    #[serde(rename = "type")]
    kind: &'static str,

    elements: Vec<RichTextElementType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<i64>,
}

impl RichTextPreformatted {
    /// Construct a [`RichTextPreformattedBuilder`].
    pub fn builder() -> RichTextPreformattedBuilder {
        RichTextPreformattedBuilder::default()
    }
}

/// Builder for [`RichTextPreformatted`] object.
#[derive(Debug, Default)]
pub struct RichTextPreformattedBuilder {
    elements: Vec<RichTextElementType>,
    border: Option<i64>,
}

impl RichTextPreformattedBuilder {
    /// Set elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextPreformatted;
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
    /// let preformatted = RichTextPreformatted::builder()
    ///     .set_elements(
    ///         vec![
    ///             RichTextElementTypeText::builder()
    ///                 .text("{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}")
    ///                 .build()
    ///                 .into()
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_preformatted",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(preformatted).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<RichTextElementType>) -> Self {
        Self { elements, ..self }
    }

    /// Add RichTextElementType object to elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextPreformatted;
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
    /// let preformatted = RichTextPreformatted::builder()
    ///     .element(
    ///         RichTextElementTypeText::builder()
    ///             .text("{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_preformatted",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "{\n  \"object\": {\n    \"description\": \"this is an example of a json object\"\n  }\n}"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(preformatted).unwrap();
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
    /// # use slack_messaging::blocks::rich_text::elements::RichTextPreformatted;
    /// let preformatted = RichTextPreformatted::builder()
    ///     .set_border(Some(0))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_preformatted",
    ///     "elements": [],
    ///     "border": 0
    /// });
    ///
    /// let json = serde_json::to_value(preformatted).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_border(self, border: Option<i64>) -> Self {
        Self { border, ..self }
    }

    /// Set border field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextPreformatted;
    /// let preformatted = RichTextPreformatted::builder()
    ///     .border(0)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_preformatted",
    ///     "elements": [],
    ///     "border": 0
    /// });
    ///
    /// let json = serde_json::to_value(preformatted).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn border(self, border: impl Into<i64>) -> Self {
        self.set_border(Some(border.into()))
    }

    /// Build a [`RichTextPreformatted`] object.
    pub fn build(self) -> RichTextPreformatted {
        RichTextPreformatted {
            kind: "rich_text_preformatted",
            elements: self.elements,
            border: self.border,
        }
    }
}