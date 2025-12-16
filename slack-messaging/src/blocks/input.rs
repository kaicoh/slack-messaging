use crate::blocks::elements::{
    Checkboxes, DatePicker, DatetimePicker, EmailInput, FileInput, MultiSelectMenuConversations,
    MultiSelectMenuExternalDataSource, MultiSelectMenuPublicChannels, MultiSelectMenuStaticOptions,
    MultiSelectMenuUsers, NumberInput, PlainTextInput, RadioButtonGroup, RichTextInput,
    SelectMenuConversations, SelectMenuExternalDataSource, SelectMenuPublicChannels,
    SelectMenuStaticOptions, SelectMenuUsers, TimePicker, UrlInput,
};
use crate::composition_objects::{Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Input block](https://docs.slack.dev/reference/block-kit/blocks/input-block)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Input;
/// use slack_messaging::blocks::elements::PlainTextInput;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let input = Input::builder()
///     .block_id("input_1")
///     .label(plain_text!("label text")?)
///     .element(
///         PlainTextInput::builder()
///             .action_id("text_area_1")
///             .multiline(true)
///             .placeholder(plain_text!("Enter some plain text.")?)
///             .build()?
///     )
///     .optional(true)
///     .build()?;
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
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "input")]
pub struct Input {
    #[builder(validate("required", "text_object::max_2000"))]
    pub(crate) label: Option<Text<Plain>>,

    #[builder(validate("required"))]
    pub(crate) element: Option<InputElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) dispatch_action: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_2000"))]
    pub(crate) hint: Option<Text<Plain>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) optional: Option<bool>,
}

/// Objects that can be an element of the [Input]'s element field.
#[derive(Debug, Clone, Serialize, PartialEq)]
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

    /// [Multi select menu of static options](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
    /// representation
    MultiSelectMenuStaticOptions(Box<MultiSelectMenuStaticOptions>),

    /// [Multi select menu of external data source](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
    /// representation
    MultiSelectMenuExternalDataSource(Box<MultiSelectMenuExternalDataSource>),

    /// [Multi select menu of users](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
    /// representation
    MultiSelectMenuUsers(Box<MultiSelectMenuUsers>),

    /// [Multi select menu of conversations](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
    /// representation
    MultiSelectMenuConversations(Box<MultiSelectMenuConversations>),

    /// [Multi select menu of public channels](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
    /// representation
    MultiSelectMenuPublicChannels(Box<MultiSelectMenuPublicChannels>),

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

    /// [Select menu of static options](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select)
    /// representation
    SelectMenuStaticOptions(Box<SelectMenuStaticOptions>),

    /// [Select menu of external data source](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
    /// representation
    SelectMenuExternalDataSource(Box<SelectMenuExternalDataSource>),

    /// [Select menu of users](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select)
    /// representation
    SelectMenuUsers(Box<SelectMenuUsers>),

    /// [Select menu of conversations](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
    /// representation
    SelectMenuConversations(Box<SelectMenuConversations>),

    /// [Select menu of public channels](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select)
    /// representation
    SelectMenuPublicChannels(Box<SelectMenuPublicChannels>),

    /// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
    /// representation
    TimePicker(Box<TimePicker>),

    /// [URL input element](https://docs.slack.dev/reference/block-kit/block-elements/url-input-element)
    /// representation
    UrlInput(Box<UrlInput>),
}

macro_rules! input_from {
    ($($ty:ident,)*) => {
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
    MultiSelectMenuStaticOptions,
    MultiSelectMenuExternalDataSource,
    MultiSelectMenuUsers,
    MultiSelectMenuConversations,
    MultiSelectMenuPublicChannels,
    NumberInput,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    SelectMenuStaticOptions,
    SelectMenuExternalDataSource,
    SelectMenuUsers,
    SelectMenuConversations,
    SelectMenuPublicChannels,
    TimePicker,
    UrlInput,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::elements::test_helpers::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Input {
            label: Some(plain_text("foo")),
            element: Some(text_input().into()),
            dispatch_action: Some(true),
            block_id: Some("input_0".into()),
            hint: Some(plain_text("bar")),
            optional: Some(true),
        };

        let val = Input::builder()
            .set_label(Some(plain_text("foo")))
            .set_element(Some(text_input()))
            .set_dispatch_action(Some(true))
            .set_block_id(Some("input_0"))
            .set_hint(Some(plain_text("bar")))
            .set_optional(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Input::builder()
            .label(plain_text("foo"))
            .element(text_input())
            .dispatch_action(true)
            .block_id("input_0")
            .hint(plain_text("bar"))
            .optional(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_label_field() {
        let err = Input::builder().element(text_input()).build().unwrap_err();
        assert_eq!(err.object(), "Input");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_label_less_than_2000_characters_long() {
        let err = Input::builder()
            .label(plain_text("a".repeat(2001)))
            .element(text_input())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Input");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(2000)));
    }

    #[test]
    fn it_requires_element_field() {
        let err = Input::builder()
            .label(plain_text("foo"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Input");

        let errors = err.field("element");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Input::builder()
            .label(plain_text("foo"))
            .element(text_input())
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Input");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_hint_less_then_2000_characters_long() {
        let err = Input::builder()
            .label(plain_text("foo"))
            .element(text_input())
            .hint(plain_text("a".repeat(2001)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Input");

        let errors = err.field("hint");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(2000)));
    }
}
