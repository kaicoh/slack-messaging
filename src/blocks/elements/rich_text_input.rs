use super::{
    composition_objects::{DispatchActionConfiguration, Text},
    RichText,
};
use serde::Serialize;

/// [Rich text input element](https://api.slack.com/reference/block-kit/block-elements#rich_text_input)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::RichText;
/// # use slack_messaging::blocks::elements::RichTextInput;
/// # use slack_messaging::blocks::rich_text::elements::RichTextSection;
/// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeText;
/// let rich_text = RichTextInput::builder()
///     .action_id("rich_text_input-action")
///     .initial_value(
///         RichText::builder()
///             .element(
///                 RichTextSection::builder()
///                     .element(
///                         RichTextElementTypeText::builder()
///                             .text("Hello")
///                             .build()
///                     )
///                     .build()
///             )
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "rich_text_input",
///     "action_id": "rich_text_input-action",
///     "initial_value": {
///         "type": "rich_text",
///         "elements": [
///             {
///                 "type": "rich_text_section",
///                 "elements": [
///                     {
///                         "type": "text",
///                         "text": "Hello"
///                     }
///                 ]
///             }
///         ]
///     }
/// });
///
/// let json = serde_json::to_value(rich_text).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextInput {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_value: Option<RichText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<Text>,
}
