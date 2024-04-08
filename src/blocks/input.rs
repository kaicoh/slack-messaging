use super::composition_objects::Text;
use super::elements::{
    Checkboxes, DatePicker, DatetimePicker, EmailInput, FileInput, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    NumberInput, PlainTextInput, RadioButtonGroup, RichTextInput, SelectConversations,
    SelectExternals, SelectPublicChannels, SelectStaticOptions, SelectUsers, TimePicker, UrlInput,
};
use serde::Serialize;

/// [Input block](https://api.slack.com/reference/block-kit/blocks#input)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Input;
/// # use slack_messaging::blocks::elements::PlainTextInput;
/// let input = Input::builder()
///     .block_id("input_1")
///     .label("label text")
///     .element(
///         PlainTextInput::builder()
///             .action_id("text_area_1")
///             .multiline(true)
///             .placeholder("Enter some plain text.")
///             .build()
///     )
///     .optional(true)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "input",
///     "block_id": "input_1",
///     "label": {
///         "type": "plain_text",
///         "text": "label text"
///     },
///     "element": {
///         "type": "plain_text_input",
///         "action_id": "text_area_1",
///         "multiline": true,
///         "placeholder": {
///             "type": "plain_text",
///             "text": "Enter some plain text."
///         }
///     },
///     "optional": true
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Input {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) label: Text,

    pub(super) element: InputElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hint: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) optional: Option<bool>,
}

/// Objects that can be an element of the [Input]'s element field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputElement {
    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

    /// [Email input element](https://api.slack.com/reference/block-kit/block-elements#email)
    /// representation
    EmailInput(Box<EmailInput>),

    /// [File input element](https://api.slack.com/reference/block-kit/block-elements#file_input)
    /// representation
    FileInput(Box<FileInput>),

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

    /// [Number input element](https://api.slack.com/reference/block-kit/block-elements#number)
    /// representation
    NumberInput(Box<NumberInput>),

    /// [Plain-text input element](https://api.slack.com/reference/block-kit/block-elements#input)
    /// representation
    PlainTextInput(Box<PlainTextInput>),

    /// [Radio buton group element](https://api.slack.com/reference/block-kit/block-elements#radio)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

    /// [Rich text input element](https://api.slack.com/reference/block-kit/block-elements#rich_text_input)
    /// representation
    RichTextInput(Box<RichTextInput>),

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

    /// [URL input element](https://api.slack.com/reference/block-kit/block-elements#url)
    /// representation
    UrlInput(Box<UrlInput>),
}

macro_rules! input_from {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for InputElement {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

input_from! {
    Checkboxes,
    DatePicker,
    DatetimePicker,
    EmailInput,
    FileInput,
    MultiSelectConversations,
    MultiSelectExternals,
    MultiSelectPublicChannels,
    MultiSelectStaticOptions,
    MultiSelectUsers,
    NumberInput,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    SelectConversations,
    SelectExternals,
    SelectPublicChannels,
    SelectStaticOptions,
    SelectUsers,
    TimePicker,
    UrlInput
}
