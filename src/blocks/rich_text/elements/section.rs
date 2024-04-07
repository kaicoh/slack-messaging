use super::types::RichTextElementType;
use serde::Serialize;

/// [Rich text section element](https://api.slack.com/reference/block-kit/blocks#rich_text_section)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::RichTextSection;
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeText, CodableStyle};
/// let section = RichTextSection::builder()
///     .element(
///         RichTextElementTypeText::builder()
///             .text("Hello there, ")
///             .build()
///     )
///     .element(
///         RichTextElementTypeText::builder()
///             .text("I am a bold rich text block!")
///             .style(
///                 CodableStyle::builder()
///                     .bold(true)
///                     .build()
///             )
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_section",
///     "elements": [
///         {
///             "type": "text",
///             "text": "Hello there, "
///         },
///         {
///             "type": "text",
///             "text": "I am a bold rich text block!",
///             "style": {
///                 "bold": true
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(section).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextSection {
    #[serde(rename = "type")]
    kind: &'static str,
    elements: Vec<RichTextElementType>,
}

impl RichTextSection {
    /// Construct a [`RichTextSectionBuilder`].
    pub fn builder() -> RichTextSectionBuilder {
        RichTextSectionBuilder::default()
    }
}

/// Builder for [`RichTextSection`] object.
#[derive(Debug, Default)]
pub struct RichTextSectionBuilder {
    elements: Vec<RichTextElementType>,
}

impl RichTextSectionBuilder {
    /// Set elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextSection;
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeText, CodableStyle};
    /// let section = RichTextSection::builder()
    ///     .set_elements(
    ///         vec![
    ///             RichTextElementTypeText::builder()
    ///                 .text("Hello there, ")
    ///                 .build()
    ///                 .into(),
    ///             RichTextElementTypeText::builder()
    ///                 .text("I am a bold rich text block!")
    ///                 .style(
    ///                     CodableStyle::builder()
    ///                         .bold(true)
    ///                         .build()
    ///                 )
    ///                 .build()
    ///                 .into(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_section",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "Hello there, "
    ///         },
    ///         {
    ///             "type": "text",
    ///             "text": "I am a bold rich text block!",
    ///             "style": {
    ///                 "bold": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<RichTextElementType>) -> Self {
        Self { elements }
    }

    /// Add RichTextElementType object to elements field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::RichTextSection;
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeText, CodableStyle};
    /// let section = RichTextSection::builder()
    ///     .element(
    ///         RichTextElementTypeText::builder()
    ///             .text("Hello there, ")
    ///             .build()
    ///     )
    ///     .element(
    ///         RichTextElementTypeText::builder()
    ///             .text("I am a bold rich text block!")
    ///             .style(
    ///                 CodableStyle::builder()
    ///                     .bold(true)
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_section",
    ///     "elements": [
    ///         {
    ///             "type": "text",
    ///             "text": "Hello there, "
    ///         },
    ///         {
    ///             "type": "text",
    ///             "text": "I am a bold rich text block!",
    ///             "style": {
    ///                 "bold": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<RichTextElementType>) -> Self {
        let Self { mut elements } = self;
        elements.push(element.into());
        Self { elements }
    }

    /// Build a [`RichTextSection`] object.
    pub fn build(self) -> RichTextSection {
        RichTextSection {
            kind: "rich_text_section",
            elements: self.elements,
        }
    }
}
