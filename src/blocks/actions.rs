use serde::Serialize;
use super::elements::{
    Button, CheckboxGroup, DatePicker, DatetimePicker, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions,
    MultiSelectUsers, OverflowMenu, RadioButtonGroup, SelectConversations,
    SelectExternals, SelectPublicChannels, SelectStaticOptions, SelectUsers,
    TimePicker,
};

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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_elements(self, elements: Vec<ActionsElement>) -> Self {
        Self { elements, ..self }
    }

    pub fn push_element<T: Into<ActionsElement>>(self, element: T) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}

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
