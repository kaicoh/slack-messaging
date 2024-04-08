/// Builder object.
pub mod builder;

/// Rich text elements
pub mod elements;

use elements::RichTextElement;
use serde::Serialize;

/// [Rich text block](https://api.slack.com/reference/block-kit/blocks#rich_text) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::RichText;
/// # use slack_messaging::blocks::rich_text::elements::{RichTextSection, RichTextElementTypeText, CodableStyle};
/// let rich_text = RichText::builder()
///     .block_id("block-0")
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementTypeText::builder()
///                     .text("Hello there, ")
///                     .build()
///             )
///             .element(
///                 RichTextElementTypeText::builder()
///                     .text("I am a bold rich text block!")
///                     .style(
///                         CodableStyle::builder()
///                             .bold(true)
///                             .build()
///                     )
///                     .build()
///             )
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text",
///     "block_id": "block-0",
///     "elements": [
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Hello there, "
///                 },
///                 {
///                     "type": "text",
///                     "text": "I am a bold rich text block!",
///                     "style": {
///                         "bold": true
///                     }
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
#[derive(Debug, Clone, Serialize)]
pub struct RichText {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<RichTextElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}
