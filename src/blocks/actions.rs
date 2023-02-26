use super::elements::{
    Button, CheckboxGroup, DatePicker, DatetimePicker, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    OverflowMenu, RadioButtonGroup, SelectConversations, SelectExternals, SelectPublicChannels,
    SelectStaticOptions, SelectUsers, TimePicker,
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
/// use slack_messaging::blocks::Actions;
/// use slack_messaging::blocks::elements::{Button, SelectStaticOptions, Opt};
/// use serde_json::json;
///
/// let actions = Actions::new()
///     .set_block_id("actions1")
///     .push_element(
///         SelectStaticOptions::new()
///             .set_action_id("select_2")
///             .placeholder("Which witch is the witchiest witch?")
///             .push_option(Opt::plain("Matilda").set_value("matilda"))
///             .push_option(Opt::plain("Glinda").set_value("glinda"))
///             .push_option(Opt::plain("Granny Weatherwax").set_value("grannyWeatherwax"))
///             .push_option(Opt::plain("Hermione").set_value("hermione"))
///     )
///     .push_element(
///         Button::new()
///             .set_action_id("button_1")
///             .set_value("cancel")
///             .text("Cancel")
///     );
///
/// let expected = json!({
///     "type": "actions",
///     "block_id": "actions1",
///     "elements": [
///         {
///             "type": "static_select",
///             "action_id": "select_2",
///             "placeholder": {
///                 "type": "plain_text",
///                 "text": "Which witch is the witchiest witch?",
///                 "emoji": true
///             },
///             "options": [
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Matilda",
///                         "emoji": true
///                     },
///                     "value": "matilda"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Glinda",
///                         "emoji": true
///                     },
///                     "value": "glinda"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Granny Weatherwax",
///                         "emoji": true
///                     },
///                     "value": "grannyWeatherwax"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Hermione",
///                         "emoji": true
///                     },
///                     "value": "hermione"
///                 }
///             ]
///         },
///         {
///             "type": "button",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Cancel",
///                 "emoji": true
///             },
///             "value": "cancel",
///             "action_id": "button_1"
///         }
///     ]
/// });
///
/// let actions_json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(actions_json, expected);
/// ```
///
/// And the following is the [2nd sample actions](https://api.slack.com/reference/block-kit/blocks#actions_examples).
///
/// ```
/// use slack_messaging::blocks::Actions;
/// use slack_messaging::blocks::elements::{Button, DatePicker, Opt, OverflowMenu};
/// use serde_json::json;
///
/// let actions = Actions::new()
///     .set_block_id("actionblock789")
///     .push_element(
///         DatePicker::new()
///             .set_action_id("datepicker123")
///             .set_initial_date("1990-04-28")
///             .placeholder("Select a date")
///     )
///     .push_element(
///         OverflowMenu::new()
///             .set_action_id("overflow")
///             .push_option(Opt::plain("*this is plain_text text*").set_value("value-0"))
///             .push_option(Opt::plain("*this is plain_text text*").set_value("value-1"))
///             .push_option(Opt::plain("*this is plain_text text*").set_value("value-2"))
///             .push_option(Opt::plain("*this is plain_text text*").set_value("value-3"))
///             .push_option(Opt::plain("*this is plain_text text*").set_value("value-4"))
///     )
///     .push_element(
///         Button::new()
///             .set_action_id("button")
///             .set_value("click_me_123")
///             .text("Click Me")
///     );
///
/// let expected = json!({
///     "type": "actions",
///     "block_id": "actionblock789",
///     "elements": [
///         {
///             "type": "datepicker",
///             "action_id": "datepicker123",
///             "initial_date": "1990-04-28",
///             "placeholder": {
///                 "type": "plain_text",
///                 "text": "Select a date",
///                 "emoji": true
///             }
///         },
///         {
///             "type": "overflow",
///             "action_id": "overflow",
///             "options": [
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*",
///                         "emoji": true
///                     },
///                     "value": "value-0"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*",
///                         "emoji": true
///                     },
///                     "value": "value-1"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*",
///                         "emoji": true
///                     },
///                     "value": "value-2"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*",
///                         "emoji": true
///                     },
///                     "value": "value-3"
///                 },
///                 {
///                     "text": {
///                         "type": "plain_text",
///                         "text": "*this is plain_text text*",
///                         "emoji": true
///                     },
///                     "value": "value-4"
///                 }
///             ]
///         },
///         {
///             "type": "button",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Click Me",
///                 "emoji": true
///             },
///             "value": "click_me_123",
///             "action_id": "button"
///         }
///     ]
/// });
///
/// let actions_json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(actions_json, expected);
/// ```
///
#[derive(Debug, Serialize)]
pub struct Actions {
    #[serde(rename = "type")]
    kind: &'static str,

    elements: Vec<ActionsElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Actions {
    fn default() -> Self {
        Self {
            kind: "actions",
            elements: vec![],
            block_id: None,
        }
    }
}

impl Actions {
    /// Constructs an Actions block.
    ///
    /// ```
    /// use slack_messaging::blocks::Actions;
    /// use serde_json::json;
    ///
    /// let actions = Actions::new();
    ///
    /// let expected = json!({
    ///     "type": "actions",
    ///     "elements": []
    /// });
    ///
    /// let actions_json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(actions_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets elements field directly. The argument is a vector composed from any objects
    /// that can transform into the enum [ActionsElement].
    ///
    /// ```
    /// use slack_messaging::blocks::Actions;
    /// use slack_messaging::blocks::elements::{Button, SelectStaticOptions, Opt};
    /// use serde_json::json;
    ///
    /// let actions = Actions::new()
    ///     .set_elements(
    ///         vec![
    ///             SelectStaticOptions::new()
    ///                 .set_action_id("select_2")
    ///                 .placeholder("Which witch is the witchiest witch?")
    ///                 .push_option(Opt::plain("Matilda").set_value("matilda"))
    ///                 .push_option(Opt::plain("Glinda").set_value("glinda"))
    ///                 .into(),
    ///             Button::new()
    ///                 .set_action_id("button_1")
    ///                 .set_value("cancel")
    ///                 .text("Cancel")
    ///                 .into(),
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "actions",
    ///     "elements": [
    ///         {
    ///             "type": "static_select",
    ///             "action_id": "select_2",
    ///             "placeholder": {
    ///                 "type": "plain_text",
    ///                 "text": "Which witch is the witchiest witch?",
    ///                 "emoji": true
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "Matilda",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "matilda"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "Glinda",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "glinda"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "button",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Cancel",
    ///                 "emoji": true
    ///             },
    ///             "value": "cancel",
    ///             "action_id": "button_1"
    ///         }
    ///     ]
    /// });
    ///
    /// let actions_json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(actions_json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<ActionsElement>) -> Self {
        Self { elements, ..self }
    }

    /// Adds an object to elements field. The argument is an any object
    /// that can transform into the enum [ActionsElement].
    ///
    /// ```
    /// use slack_messaging::blocks::Actions;
    /// use slack_messaging::blocks::elements::{Button, DatePicker};
    /// use serde_json::json;
    ///
    /// let actions = Actions::new()
    ///     .push_element(
    ///         DatePicker::new()
    ///             .set_action_id("datepicker123")
    ///             .set_initial_date("1990-04-28")
    ///             .placeholder("Select a date")
    ///     )
    ///     .push_element(
    ///         Button::new()
    ///             .set_action_id("button")
    ///             .set_value("click_me_123")
    ///             .text("Click Me")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "actions",
    ///     "elements": [
    ///         {
    ///             "type": "datepicker",
    ///             "action_id": "datepicker123",
    ///             "initial_date": "1990-04-28",
    ///             "placeholder": {
    ///                 "type": "plain_text",
    ///                 "text": "Select a date",
    ///                 "emoji": true
    ///             }
    ///         },
    ///         {
    ///             "type": "button",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Click Me",
    ///                 "emoji": true
    ///             },
    ///             "value": "click_me_123",
    ///             "action_id": "button"
    ///         }
    ///     ]
    /// });
    ///
    /// let actions_json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(actions_json, expected);
    /// ```
    pub fn push_element<T: Into<ActionsElement>>(self, element: T) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Sets block_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::Actions;
    /// use serde_json::json;
    ///
    /// let actions = Actions::new().set_block_id("actions_block_1");
    ///
    /// let expected = json!({
    ///     "type": "actions",
    ///     "elements": [],
    ///     "block_id": "actions_block_1"
    /// });
    ///
    /// let actions_json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(actions_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}

/// Objects that can be an element of the [Actions]'s elements field.
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ActionsElement {
    Button(Box<Button>),
    CheckboxGroup(Box<CheckboxGroup>),
    DatePicker(Box<DatePicker>),
    DatetimePicker(Box<DatetimePicker>),
    MultiSelectConversations(Box<MultiSelectConversations>),
    MultiSelectExternals(Box<MultiSelectExternals>),
    MultiSelectPublicChannels(Box<MultiSelectPublicChannels>),
    MultiSelectStaticOptions(Box<MultiSelectStaticOptions>),
    MultiSelectUsers(Box<MultiSelectUsers>),
    OverflowMenu(Box<OverflowMenu>),
    RadioButtonGroup(Box<RadioButtonGroup>),
    SelectConversations(Box<SelectConversations>),
    SelectExternals(Box<SelectExternals>),
    SelectPublicChannels(Box<SelectPublicChannels>),
    SelectStaticOptions(Box<SelectStaticOptions>),
    SelectUsers(Box<SelectUsers>),
    TimePicker(Box<TimePicker>),
}

impl From<Button> for ActionsElement {
    fn from(value: Button) -> Self {
        Self::Button(Box::new(value))
    }
}

impl From<CheckboxGroup> for ActionsElement {
    fn from(value: CheckboxGroup) -> Self {
        Self::CheckboxGroup(Box::new(value))
    }
}

impl From<DatePicker> for ActionsElement {
    fn from(value: DatePicker) -> Self {
        Self::DatePicker(Box::new(value))
    }
}

impl From<DatetimePicker> for ActionsElement {
    fn from(value: DatetimePicker) -> Self {
        Self::DatetimePicker(Box::new(value))
    }
}

impl From<MultiSelectConversations> for ActionsElement {
    fn from(value: MultiSelectConversations) -> Self {
        Self::MultiSelectConversations(Box::new(value))
    }
}

impl From<MultiSelectExternals> for ActionsElement {
    fn from(value: MultiSelectExternals) -> Self {
        Self::MultiSelectExternals(Box::new(value))
    }
}

impl From<MultiSelectPublicChannels> for ActionsElement {
    fn from(value: MultiSelectPublicChannels) -> Self {
        Self::MultiSelectPublicChannels(Box::new(value))
    }
}

impl From<MultiSelectStaticOptions> for ActionsElement {
    fn from(value: MultiSelectStaticOptions) -> Self {
        Self::MultiSelectStaticOptions(Box::new(value))
    }
}

impl From<MultiSelectUsers> for ActionsElement {
    fn from(value: MultiSelectUsers) -> Self {
        Self::MultiSelectUsers(Box::new(value))
    }
}

impl From<OverflowMenu> for ActionsElement {
    fn from(value: OverflowMenu) -> Self {
        Self::OverflowMenu(Box::new(value))
    }
}

impl From<RadioButtonGroup> for ActionsElement {
    fn from(value: RadioButtonGroup) -> Self {
        Self::RadioButtonGroup(Box::new(value))
    }
}

impl From<SelectConversations> for ActionsElement {
    fn from(value: SelectConversations) -> Self {
        Self::SelectConversations(Box::new(value))
    }
}

impl From<SelectExternals> for ActionsElement {
    fn from(value: SelectExternals) -> Self {
        Self::SelectExternals(Box::new(value))
    }
}

impl From<SelectPublicChannels> for ActionsElement {
    fn from(value: SelectPublicChannels) -> Self {
        Self::SelectPublicChannels(Box::new(value))
    }
}

impl From<SelectStaticOptions> for ActionsElement {
    fn from(value: SelectStaticOptions) -> Self {
        Self::SelectStaticOptions(Box::new(value))
    }
}

impl From<SelectUsers> for ActionsElement {
    fn from(value: SelectUsers) -> Self {
        Self::SelectUsers(Box::new(value))
    }
}

impl From<TimePicker> for ActionsElement {
    fn from(value: TimePicker) -> Self {
        Self::TimePicker(Box::new(value))
    }
}
