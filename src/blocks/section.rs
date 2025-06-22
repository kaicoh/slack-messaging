use super::composition_objects::Text;
use super::elements::{
    Button, Checkboxes, DatePicker, Image, MultiSelectMenu, OverflowMenu, RadioButtonGroup,
    SelectMenu, TimePicker, WorkflowButton,
};
use serde::Serialize;

/// [Section block](https://docs.slack.dev/reference/block-kit/blocks/section-block)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Section;
/// # use slack_messaging::blocks::elements::Image;
/// # use slack_messaging::{mrkdwn, plain_text};
/// let section = Section::builder()
///     .block_id("section_1")
///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
///     .field(mrkdwn!("High"))
///     .field(plain_text!("String"))
///     .accessory(
///         Image::builder()
///             .image_url("http://placekitten.com/700/500")
///             .alt_text("Multiple cute kittens")
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "section",
///     "block_id": "section_1",
///     "text": {
///         "type": "mrkdwn",
///         "text": "A message *with some bold text* and _some italicized text_."
///     },
///     "fields": [
///         {
///             "type": "mrkdwn",
///             "text": "High"
///         },
///         {
///             "type": "plain_text",
///             "text": "String"
///         }
///     ],
///     "accessory": {
///         "type": "image",
///         "image_url": "http://placekitten.com/700/500",
///         "alt_text": "Multiple cute kittens"
///     }
/// });
///
/// let json = serde_json::to_value(section).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Section {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) text: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) fields: Vec<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) accessory: Option<Accessory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) expand: Option<bool>,
}

/// Objects that can be set to [Section] as an accessory.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Accessory {
    /// [Button element](https://docs.slack.dev/reference/block-kit/block-elements/button-element)
    /// representation
    Button(Box<Button>),

    /// [Checkbox group](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Image element](https://docs.slack.dev/reference/block-kit/block-elements/image-element)
    /// representation
    Image(Box<Image>),

    /// [Multi-select menu element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element)
    /// representation
    MultiSelectMenu(Box<MultiSelectMenu>),

    /// [Overflow menu element](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element)
    /// representation
    OverflowMenu(Box<OverflowMenu>),

    /// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

    /// [Select menu element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
    /// representation
    SelectMenu(Box<SelectMenu>),

    /// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
    /// representation
    TimePicker(Box<TimePicker>),

    /// [Workflow button element](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element)
    /// representation
    WorkflowButton(Box<WorkflowButton>),
}

macro_rules! accessory_from {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for Accessory {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

accessory_from! {
    Button,
    Checkboxes,
    DatePicker,
    Image,
    MultiSelectMenu,
    OverflowMenu,
    RadioButtonGroup,
    SelectMenu,
    TimePicker,
    WorkflowButton
}
