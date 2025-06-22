use super::elements::{
    Button, Checkboxes, DatePicker, DatetimePicker, MultiSelectMenu, OverflowMenu,
    RadioButtonGroup, SelectMenu, TimePicker, WorkflowButton,
};
use serde::Serialize;

/// [Actions block](https://docs.slack.dev/reference/block-kit/blocks/actions-block)
/// representation.
///
/// # Example
///
/// The following is reproduction of [the 1st sample actions](https://docs.slack.dev/reference/block-kit/blocks/actions-block#examples).
///
/// ```
/// # use slack_messaging::blocks::Actions;
/// # use slack_messaging::blocks::elements::{Button, SelectStaticOptions};
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
/// let actions = Actions::builder()
///     .block_id("actions1")
///     .element(
///         SelectStaticOptions::builder()
///             .action_id("select_2")
///             .placeholder("Which witch is the witchiest witch?")
///             .set_options(
///                 vec![
///                     Opt::builder().text(plain_text!("Matilda")).value("matilda").build(),
///                     Opt::builder().text(plain_text!("Glinda")).value("glinda").build(),
///                     Opt::builder().text(plain_text!("Granny Weatherwax")).value("grannyWeatherwax").build(),
///                     Opt::builder().text(plain_text!("Hermione")).value("hermione").build(),
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
/// And the following is the [2nd sample actions](https://docs.slack.dev/reference/block-kit/blocks/actions-block#examples).
///
/// ```
/// # use slack_messaging::blocks::Actions;
/// # use slack_messaging::blocks::elements::{Button, DatePicker, OverflowMenu};
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
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
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*"))
///                     .value("value-0")
///                     .build()
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*"))
///                     .value("value-1")
///                     .build()
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*"))
///                     .value("value-2")
///                     .build()
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*"))
///                     .value("value-3")
///                     .build()
///             )
///             .option(
///                 Opt::builder()
///                     .text(plain_text!("*this is plain_text text*"))
///                     .value("value-4")
///                     .build()
///             )
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

    /// [Multi-select menu element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element)
    /// representation
    MultiSelectMenu(Box<MultiSelectMenu>),

    /// [Overflow menu element](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element)
    /// representation
    OverflowMenu(Box<OverflowMenu>),

    /// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
    /// representation
    RadioButtonGroup(Box<RadioButtonGroup>),

    /// [Select menu element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element)
    /// representation
    SelectMenu(Box<SelectMenu>),

    /// [Time picker element](https://docs.slack.dev/reference/block-kit/block-elements/time-picker-element)
    /// representation
    TimePicker(Box<TimePicker>),

    /// [Workflow button element](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element)
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
    MultiSelectMenu,
    OverflowMenu,
    RadioButtonGroup,
    SelectMenu,
    TimePicker,
    WorkflowButton
}
