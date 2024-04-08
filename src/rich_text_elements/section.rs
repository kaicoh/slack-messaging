use super::RichTextElementType;
use serde::Serialize;

/// [Rich text section element](https://api.slack.com/reference/block-kit/blocks#rich_text_section)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::rich_text_elements::{RichTextSection, RichTextElementTypeText,
/// CodableStyle};
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
    pub(super) kind: &'static str,
    pub(super) elements: Vec<RichTextElementType>,
}
