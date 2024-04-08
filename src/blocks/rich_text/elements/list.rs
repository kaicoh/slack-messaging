use super::RichTextSection;
use serde::Serialize;

/// [Rich text list element](https://api.slack.com/reference/block-kit/blocks#rich_text_list)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::{RichTextList,
/// RichTextListStyle, RichTextSection, RichTextElementTypeText};
/// let list = RichTextList::builder()
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementTypeText::builder()
///                     .text("Huddles")
///                     .build()
///             )
///             .build()
///     )
///     .element(
///         RichTextSection::builder()
///             .element(
///                 RichTextElementTypeText::builder()
///                     .text("Canvas")
///                     .build()
///             )
///             .build()
///     )
///     .style(RichTextListStyle::Bullet)
///     .indent(0)
///     .border(1)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_list",
///     "elements": [
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Huddles",
///                 }
///             ]
///         },
///         {
///             "type": "rich_text_section",
///             "elements": [
///                 {
///                     "type": "text",
///                     "text": "Canvas",
///                 }
///             ]
///         }
///     ],
///     "style": "bullet",
///     "indent": 0,
///     "border": 1
/// });
///
/// let json = serde_json::to_value(list).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextList {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) style: RichTextListStyle,

    pub(super) elements: Vec<RichTextSection>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) indent: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) offset: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) border: Option<i64>,
}

/// List style for [`RichTextList`].
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RichTextListStyle {
    Bullet,
    Ordered,
}
