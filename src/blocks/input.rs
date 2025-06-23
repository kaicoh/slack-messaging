use super::composition_objects::PlainText;
use super::elements::{
    Checkboxes, DatePicker, DatetimePicker, EmailInput, FileInput, MultiSelectMenu, NumberInput,
    PlainTextInput, RadioButtonGroup, RichTextInput, SelectMenu, TimePicker, UrlInput,
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

    pub(super) label: PlainText,

    pub(super) element: InputElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) hint: Option<PlainText>,

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

    /// [Multi-select menu element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element)
    /// representation
    MultiSelectMenu(Box<MultiSelectMenu>),

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

    /// [Select menu element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element)
    /// representation
    SelectMenu(Box<SelectMenu>),

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
    MultiSelectMenu,
    NumberInput,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    SelectMenu,
    TimePicker,
    UrlInput
}
