use super::rich_text_elements::{
    RichTextList, RichTextPreformatted, RichTextQuote, RichTextSection,
};
use serde::Serialize;

/// [Rich text block](https://api.slack.com/reference/block-kit/blocks#rich_text) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::RichText;
/// # use slack_messaging::rich_text_elements::{RichTextSection, RichTextElementTypeText, CodableStyle};
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

/// Objects that can be an element of the [RichText]’s elements field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichTextElement {
    /// [Section element](https://api.slack.com/reference/block-kit/blocks#rich_text_section)
    /// representation
    Section(Box<RichTextSection>),

    /// [List element](https://api.slack.com/reference/block-kit/blocks#rich_text_list)
    /// representation
    List(Box<RichTextList>),

    /// [Preformatted element](https://api.slack.com/reference/block-kit/blocks#rich_text_preformatted)
    /// representation
    Preformatted(Box<RichTextPreformatted>),

    /// [Quote element](https://api.slack.com/reference/block-kit/blocks#rich_text_quote)
    /// representation
    Quote(Box<RichTextQuote>),
}

impl From<RichTextSection> for RichTextElement {
    fn from(value: RichTextSection) -> Self {
        Self::Section(Box::new(value))
    }
}

impl From<RichTextList> for RichTextElement {
    fn from(value: RichTextList) -> Self {
        Self::List(Box::new(value))
    }
}

impl From<RichTextPreformatted> for RichTextElement {
    fn from(value: RichTextPreformatted) -> Self {
        Self::Preformatted(Box::new(value))
    }
}

impl From<RichTextQuote> for RichTextElement {
    fn from(value: RichTextQuote) -> Self {
        Self::Quote(Box::new(value))
    }
}
