use super::composition_objects::Text;
use super::elements::{
    Button, Checkboxes, DatePicker, Image, MultiSelectConversations, MultiSelectExternals,
    MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers, OverflowMenu,
    RadioButtonGroup, SelectConversations, SelectExternals, SelectPublicChannels,
    SelectStaticOptions, SelectUsers, TimePicker, WorkflowButton,
};
use serde::Serialize;

/// [Section block](https://api.slack.com/reference/block-kit/blocks#section)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Section;
/// # use slack_messaging::blocks::elements::Image;
/// # use slack_messaging::composition_objects::Text;
/// let section = Section::builder()
///     .block_id("section_1")
///     .text(
///         Text::builder()
///             .mrkdwn("A message *with some bold text* and _some italicized text_.")
///             .build()
///     )
///     .field(Text::builder().mrkdwn("High").build())
///     .field(Text::builder().plain_text("String").build())
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
}

/// Objects that can be set to [Section] as an accessory.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Accessory {
    /// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
    /// representation
    Button(Box<Button>),

    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Image element](https://api.slack.com/reference/block-kit/block-elements#image)
    /// representation
    Image(Box<Image>),

    /// [Multi-select menu Conversations list element](https://api.slack.com/reference/block-kit/block-elements#conversation_multi_select)
    /// representation
    MultiSelectConversations(Box<MultiSelectConversations>),

    /// [Multi-select menu External data source element](https://api.slack.com/reference/block-kit/block-elements#external_multi_select)
    /// representation
    MultiSelectExternals(Box<MultiSelectExternals>),

    /// [Multi-select menu Public channels element](https://api.slack.com/reference/block-kit/block-elements#channel_multi_select)
    /// representation
    MultiSelectPublicChannels(Box<MultiSelectPublicChannels>),

    /// [Multi-select menu Static options element](https://api.slack.com/reference/block-kit/block-elements#static_multi_select)
    /// representation
    MultiSelectStaticOptions(Box<MultiSelectStaticOptions>),

    /// [Multi-select menu User list element](https://api.slack.com/reference/block-kit/block-elements#users_multi_select)
    /// representation
    MultiSelectUsers(Box<MultiSelectUsers>),

    /// [Overflow menu element](https://api.slack.com/reference/block-kit/block-elements#overflow)
    /// representation
    OverflowMenu(Box<OverflowMenu>),

    /// [Radio buton group element](https://api.slack.com/reference/block-kit/block-elements#radio)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

    /// [Select menu of conversations element](https://api.slack.com/reference/block-kit/block-elements#conversations_select)
    /// representation
    SelectConversations(Box<SelectConversations>),

    /// [Select menu of external data source element](https://api.slack.com/reference/block-kit/block-elements#external_select)
    /// representation
    SelectExternals(Box<SelectExternals>),

    /// [Select menu of public channels element](https://api.slack.com/reference/block-kit/block-elements#conversations_select)
    /// representation
    SelectPublicChannels(Box<SelectPublicChannels>),

    /// [Select menu of static options element](https://api.slack.com/reference/block-kit/block-elements#static_select)
    /// representation
    SelectStaticOptions(Box<SelectStaticOptions>),

    /// [Select menu of users element](https://api.slack.com/reference/block-kit/block-elements#users_select)
    /// representation
    SelectUsers(Box<SelectUsers>),

    /// [Time picker element](https://api.slack.com/reference/block-kit/block-elements#timepicker)
    /// representation
    TimePicker(Box<TimePicker>),

    /// [Workflow button element](https://api.slack.com/reference/block-kit/block-elements#workflow_button)
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
    MultiSelectConversations,
    MultiSelectExternals,
    MultiSelectPublicChannels,
    MultiSelectStaticOptions,
    MultiSelectUsers,
    OverflowMenu,
    RadioButtonGroup,
    SelectConversations,
    SelectExternals,
    SelectPublicChannels,
    SelectStaticOptions,
    SelectUsers,
    TimePicker,
    WorkflowButton
}
