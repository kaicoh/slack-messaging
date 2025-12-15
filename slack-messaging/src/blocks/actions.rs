use crate::blocks::elements::{
    Button, Checkboxes, DatePicker, DatetimePicker, MultiSelectMenuConversations,
    MultiSelectMenuExternalDataSource, MultiSelectMenuPublicChannels, MultiSelectMenuStaticOptions,
    MultiSelectMenuUsers, OverflowMenu, RadioButtonGroup, SelectMenuConversations,
    SelectMenuExternalDataSource, SelectMenuPublicChannels, SelectMenuStaticOptions,
    SelectMenuUsers, TimePicker, WorkflowButton,
};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Actions block](https://docs.slack.dev/reference/block-kit/blocks/actions-block)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the 1st sample actions](https://docs.slack.dev/reference/block-kit/blocks/actions-block#examples).
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Actions;
/// use slack_messaging::blocks::elements::{Button, SelectMenuStaticOptions};
/// use slack_messaging::composition_objects::Opt;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let actions = Actions::builder()
///     .block_id("actions1")
///     .element(
///         SelectMenuStaticOptions::builder()
///             .action_id("select_2")
///             .placeholder(plain_text!("Which witch is the witchiest witch?")?)
///             .options(
///                 vec![
///                     Opt::builder()
///                         .text(plain_text!("Matilda")?)
///                         .value("matilda")
///                         .build()?,
///                     Opt::builder()
///                         .text(plain_text!("Glinda")?)
///                         .value("glinda")
///                         .build()?,
///                     Opt::builder()
///                         .text(plain_text!("Granny Weatherwax")?)
///                         .value("grannyWeatherwax")
///                         .build()?,
///                     Opt::builder()
///                         .text(plain_text!("Hermione")?)
///                         .value("hermione")
///                         .build()?,
///                 ]
///             )
///             .build()?
///     )
///     .element(
///         Button::builder()
///             .action_id("button_1")
///             .value("cancel")
///             .text(plain_text!("Cancel")?)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "actions",
///     "block_id": "actions1",
///     "elements": [
///         {
///             "type": "static_select",
///             "action_id": "select_2",
///             "placeholder": {
///                 "type": "plain_text",
///                 "text": "Which witch is the witchiest witch?"
///             },
///             "options": [
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Matilda"
///                     },
///                     "value": "matilda"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Glinda"
///                     },
///                     "value": "glinda"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Granny Weatherwax"
///                     },
///                     "value": "grannyWeatherwax"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Hermione"
///                     },
///                     "value": "hermione"
///                 }
///             ]
///         },
///         {
///             "type": "button",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Cancel"
///             },
///             "value": "cancel",
///             "action_id": "button_1"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// And the following is the [2nd sample actions](https://docs.slack.dev/reference/block-kit/blocks/actions-block#examples).
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::Actions;
/// use slack_messaging::blocks::elements::{Button, DatePicker, OverflowMenu};
/// use slack_messaging::composition_objects::Opt;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let actions = Actions::builder()
///     .block_id("actionblock789")
///     .element(
///         DatePicker::builder()
///             .action_id("datepicker123")
///             .initial_date("1990-04-28")
///             .placeholder(plain_text!("Select a date")?)
///             .build()?
///     )
///     .element(
///         OverflowMenu::builder()
///             .action_id("overflow")
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*")?)
///                     .value("value-0")
///                     .build()?
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*")?)
///                     .value("value-1")
///                     .build()?
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*")?)
///                     .value("value-2")
///                     .build()?
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*")?)
///                     .value("value-3")
///                     .build()?
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*")?)
///                     .value("value-4")
///                     .build()?
///             )
///             .build()?
///     )
///     .element(
///         Button::builder()
///             .action_id("button")
///             .value("click_me_123")
///             .text(plain_text!("Click Me")?)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "actions",
///     "block_id": "actionblock789",
///     "elements": [
///         {
///             "type": "datepicker",
///             "action_id": "datepicker123",
///             "initial_date": "1990-04-28",
///             "placeholder": {
///                 "type": "plain_text",
///                 "text": "Select a date"
///             }
///         },
///         {
///             "type": "overflow",
///             "action_id": "overflow",
///             "options": [
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*"
///                     },
///                     "value": "value-0"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*"
///                     },
///                     "value": "value-1"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*"
///                     },
///                     "value": "value-2"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*"
///                     },
///                     "value": "value-3"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*"
///                     },
///                     "value": "value-4"
///                 }
///             ]
///         },
///         {
///             "type": "button",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Click Me"
///             },
///             "value": "click_me_123",
///             "action_id": "button"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "actions")]
pub struct Actions {
    #[builder(push_item = "element", validate("required", "list::max_item_25"))]
    pub(crate) elements: Option<Vec<ActionsElement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

/// Objects that can be an element of the [Actions]'s elements field.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ActionsElement {
    /// [Button element](https://docs.slack.dev/reference/block-kit/block-elements/button-element)
    /// representation
    Button(Box<Button>),

    /// [Checkbox group](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://docs.slack.dev/reference/block-kit/block-elements/date-picker-element)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://docs.slack.dev/reference/block-kit/block-elements/datetime-picker-element)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

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

macro_rules! actions_from {
    ($($ty:ident,)*) => {
        $(
            impl From<$ty> for ActionsElement {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

actions_from! {
    Button,
    Checkboxes,
    DatePicker,
    DatetimePicker,
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
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Actions {
            block_id: Some("actions_0".into()),
            elements: Some(vec![datepicker().into(), btn("button_0", "value_0").into()]),
        };

        let val = Actions::builder()
            .set_block_id(Some("actions_0"))
            .set_elements(Some(vec![
                datepicker().into(),
                btn("button_0", "value_0").into(),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Actions::builder()
            .block_id("actions_0")
            .elements(vec![datepicker().into(), btn("button_0", "value_0").into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Actions {
            block_id: None,
            elements: Some(vec![datepicker().into(), btn("button_0", "value_0").into()]),
        };

        let val = Actions::builder()
            .element(datepicker())
            .element(btn("button_0", "value_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requries_elements_field() {
        let err = Actions::builder().build().unwrap_err();
        assert_eq!(err.object(), "Actions");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_elements_list_size_less_than_25() {
        let elements: Vec<ActionsElement> = (0..26).map(|_| btn("name", "value").into()).collect();
        let err = Actions::builder().elements(elements).build().unwrap_err();
        assert_eq!(err.object(), "Actions");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(25)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Actions::builder()
            .block_id("a".repeat(256))
            .element(datepicker())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Actions");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
