use crate::blocks::elements::{
    Button, Checkboxes, DatePicker, Image, MultiSelectMenuConversations,
    MultiSelectMenuExternalDataSource, MultiSelectMenuPublicChannels, MultiSelectMenuStaticOptions,
    MultiSelectMenuUsers, OverflowMenu, RadioButtonGroup, SelectMenuConversations,
    SelectMenuExternalDataSource, SelectMenuPublicChannels, SelectMenuStaticOptions,
    SelectMenuUsers, TimePicker, WorkflowButton,
};
use crate::composition_objects::TextContent;
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Section block](https://docs.slack.dev/reference/block-kit/blocks/section-block)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::{mrkdwn, plain_text};
/// use slack_messaging::blocks::Section;
/// use slack_messaging::blocks::elements::Image;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let section = Section::builder()
///     .block_id("section_1")
///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_.")?)
///     .field(mrkdwn!("High")?)
///     .field(plain_text!("String")?)
///     .accessory(
///         Image::builder()
///             .image_url("http://placekitten.com/700/500")
///             .alt_text("Multiple cute kittens")
///             .build()?
///     )
///     .build()?;
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
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "section")]
#[builder(validate = "validate")]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::min_1", "text_object::max_3000"))]
    pub(crate) text: Option<TextContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(
        push_item = "field",
        validate("list::max_item_10", "list::each_text_max_2000")
    )]
    pub(crate) fields: Option<Vec<TextContent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) accessory: Option<Accessory>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expand: Option<bool>,
}

fn validate(val: &Section) -> Vec<ValidationErrorKind> {
    match (val.text.as_ref(), val.fields.as_ref()) {
        (None, None) => {
            vec![ValidationErrorKind::EitherRequired("text", "fields")]
        }
        _ => vec![],
    }
}

/// Objects that can be set to [Section] as an accessory.
#[derive(Debug, Clone, Serialize, PartialEq)]
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

    /// [Overflow menu element](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element)
    /// representation
    OverflowMenu(Box<OverflowMenu>),

    /// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

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

    /// [Workflow button element](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element)
    /// representation
    WorkflowButton(Box<WorkflowButton>),
}

macro_rules! accessory_from {
    ($($ty:ident,)*) => {
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
    MultiSelectMenuStaticOptions,
    MultiSelectMenuExternalDataSource,
    MultiSelectMenuUsers,
    MultiSelectMenuConversations,
    MultiSelectMenuPublicChannels,
    OverflowMenu,
    RadioButtonGroup,
    SelectMenuStaticOptions,
    SelectMenuExternalDataSource,
    SelectMenuUsers,
    SelectMenuConversations,
    SelectMenuPublicChannels,
    TimePicker,
    WorkflowButton,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::elements::test_helpers::*;
    use crate::composition_objects::test_helpers::*;

    #[test]
    fn it_implements_builder() {
        let expected = Section {
            text: Some(mrkdwn_text("foo").into()),
            block_id: Some("section_0".into()),
            fields: Some(vec![plain_text("bar").into(), mrkdwn_text("baz").into()]),
            accessory: Some(btn("btn0", "val0").into()),
            expand: Some(true),
        };

        let val = Section::builder()
            .set_text(Some(mrkdwn_text("foo")))
            .set_block_id(Some("section_0"))
            .set_fields(Some(vec![
                plain_text("bar").into(),
                mrkdwn_text("baz").into(),
            ]))
            .set_accessory(Some(btn("btn0", "val0")))
            .set_expand(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Section::builder()
            .text(mrkdwn_text("foo"))
            .block_id("section_0")
            .fields(vec![plain_text("bar").into(), mrkdwn_text("baz").into()])
            .accessory(btn("btn0", "val0"))
            .expand(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Section {
            text: None,
            block_id: None,
            fields: Some(vec![plain_text("bar").into(), mrkdwn_text("baz").into()]),
            accessory: None,
            expand: None,
        };

        let val = Section::builder()
            .field(plain_text("bar"))
            .field(mrkdwn_text("baz"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_more_than_1_character_long() {
        let err = Section::builder()
            .text(mrkdwn_text(""))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MinTextLength(1)));
    }

    #[test]
    fn it_requires_text_less_than_3000_characters_long() {
        let err = Section::builder()
            .text(mrkdwn_text("a".repeat(3001)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(3000)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Section::builder()
            .text(mrkdwn_text("foo"))
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_fields_list_size_less_than_10() {
        let fields: Vec<TextContent> = (0..11).map(|_| plain_text("foobar").into()).collect();
        let err = Section::builder().fields(fields).build().unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.field("fields");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(10)));
    }

    #[test]
    fn it_requires_each_field_text_less_than_2000_characters_long() {
        let err = Section::builder()
            .field(mrkdwn_text("a".repeat(2001)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.field("fields");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(2000)));
    }

    #[test]
    fn it_prevents_from_both_text_and_fields_are_not_set() {
        let err = Section::builder().build().unwrap_err();
        assert_eq!(err.object(), "Section");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::EitherRequired("text", "fields")));
    }
}
