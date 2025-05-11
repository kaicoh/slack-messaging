use super::composition_objects::Text;
use super::elements::{
    Checkboxes, DatePicker, DatetimePicker, EmailInput, FileInput, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    NumberInput, PlainTextInput, RadioButtonGroup, RichTextInput, SelectConversations,
    SelectExternals, SelectPublicChannels, SelectStaticOptions, SelectUsers, TimePicker, UrlInput,
};
use serde::Serialize;

/// [Input block](https://docs.slack.dev/reference/block-kit/blocks/input-block)
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
    /// [Checkbox group](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

    /// [Email input element](https://docs.slack.dev/reference/block-kit/block-elements/email-input-element)
    /// representation
    EmailInput(Box<EmailInput>),

    /// [File input element](https://docs.slack.dev/reference/block-kit/block-elements/file-input-element)
    /// representation
    FileInput(Box<FileInput>),

    /// [Multi-select menu Conversations list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
    /// representation
    MultiSelectConversations(Box<MultiSelectConversations>),

    /// [Multi-select menu External data source element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
    /// representation
    MultiSelectExternals(Box<MultiSelectExternals>),

    /// [Multi-select menu Public channels element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
    /// representation
    MultiSelectPublicChannels(Box<MultiSelectPublicChannels>),

    /// [Multi-select menu Static options element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
    /// representation
    MultiSelectStaticOptions(Box<MultiSelectStaticOptions>),

    /// [Multi-select menu User list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
    /// representation
    MultiSelectUsers(Box<MultiSelectUsers>),

    /// [Number input element](https://docs.slack.dev/reference/block-kit/block-elements/number-input-element)
    /// representation
    NumberInput(Box<NumberInput>),

    /// [Plain-text input element](https://docs.slack.dev/reference/block-kit/block-elements/plain-text-input-element)
    /// representation
    PlainTextInput(Box<PlainTextInput>),

    /// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

    /// [Rich text input element](https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element)
    /// representation
    RichTextInput(Box<RichTextInput>),

    /// [Select menu of conversations element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
    /// representation
    SelectConversations(Box<SelectConversations>),

    /// [Select menu of external data source element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
    /// representation
    SelectExternals(Box<SelectExternals>),

    /// [Select menu of public channels element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select)
    /// representation
    SelectPublicChannels(Box<SelectPublicChannels>),

    /// [Select menu of static options element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select)
    /// representation
    SelectStaticOptions(Box<SelectStaticOptions>),

    /// [Select menu of users element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select)
    /// representation
    SelectUsers(Box<SelectUsers>),

    /// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
    /// representation
    TimePicker(Box<TimePicker>),

    /// [URL input element](https://docs.slack.dev/reference/block-kit/block-elements/url-input-element)
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
