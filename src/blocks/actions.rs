use super::elements::{
    Button, Checkboxes, DatePicker, DatetimePicker, MultiSelectConversations, MultiSelectExternals,
    MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers, OverflowMenu,
    RadioButtonGroup, SelectConversations, SelectExternals, SelectPublicChannels,
    SelectStaticOptions, SelectUsers, TimePicker, WorkflowButton,
};
use serde::Serialize;

/// [Actions block](https://api.slack.com/reference/block-kit/blocks#actions)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the 1st sample actions](https://api.slack.com/reference/block-kit/blocks#actions_examples).
///
/// ```
/// # use slack_messaging::blocks::Actions;
/// # use slack_messaging::blocks::elements::{Button, SelectStaticOptions};
/// # use slack_messaging::composition_objects::Opt;
/// let actions = Actions::builder()
///     .block_id("actions1")
///     .element(
///         SelectStaticOptions::builder()
///             .action_id("select_2")
///             .placeholder("Which witch is the witchiest witch?")
///             .set_options(
///                 vec![
///                     Opt::builder().text("Matilda").value("matilda").build(),
///                     Opt::builder().text("Glinda").value("glinda").build(),
///                     Opt::builder().text("Granny Weatherwax").value("grannyWeatherwax").build(),
///                     Opt::builder().text("Hermione").value("hermione").build(),
///                 ]
///             )
///             .build()
///     )
///     .element(
///         Button::builder()
///             .action_id("button_1")
///             .value("cancel")
///             .text("Cancel")
///             .build()
///     )
///     .build();
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
/// ```
///
/// And the following is the [2nd sample actions](https://api.slack.com/reference/block-kit/blocks#actions_examples).
///
/// ```
/// # use slack_messaging::blocks::Actions;
/// # use slack_messaging::blocks::elements::{Button, DatePicker, OverflowMenu};
/// # use slack_messaging::composition_objects::Opt;
/// let actions = Actions::builder()
///     .block_id("actionblock789")
///     .element(
///         DatePicker::builder()
///             .action_id("datepicker123")
///             .initial_date("1990-04-28")
///             .placeholder("Select a date")
///             .build()
///     )
///     .element(
///         OverflowMenu::builder()
///             .action_id("overflow")
///             .option(Opt::builder().text("*this is plain_text text*").value("value-0").build())
///             .option(Opt::builder().text("*this is plain_text text*").value("value-1").build())
///             .option(Opt::builder().text("*this is plain_text text*").value("value-2").build())
///             .option(Opt::builder().text("*this is plain_text text*").value("value-3").build())
///             .option(Opt::builder().text("*this is plain_text text*").value("value-4").build())
///             .build()
///     )
///     .element(
///         Button::builder()
///             .action_id("button")
///             .value("click_me_123")
///             .text("Click Me")
///             .build()
///     )
///     .build();
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
/// ```
///
#[derive(Debug, Clone, Serialize)]
pub struct Actions {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<ActionsElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}

/// Objects that can be an element of the [Actions]'s elements field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ActionsElement {
    /// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
    /// representation
    Button(Box<Button>),

    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

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

macro_rules! actions_from {
    ($($ty:ident),*) => {
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
