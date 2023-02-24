use super::elements::{
    CheckboxGroup, DatePicker, DatetimePicker, EmailInput, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    NumberInput, PlainTextInput, RadioButtonGroup, SelectConversations, SelectExternals,
    SelectPublicChannels, SelectStaticOptions, SelectUsers, Text, TimePicker, UrlInput,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Input {
    #[serde(rename = "type")]
    kind: &'static str,

    label: Text,

    element: Option<InputElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hint: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<bool>,
}

impl Default for Input {
    fn default() -> Self {
        Self {
            kind: "input",
            label: Text::plain(""),
            element: None,
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
        }
    }
}

impl Input {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self::default().label(label)
    }

    pub fn set_label(self, label: Text) -> Self {
        Self { label, ..self }
    }

    pub fn label<T: Into<String>>(self, label: T) -> Self {
        self.set_label(Text::plain(label))
    }

    pub fn set_element<T: Into<InputElement>>(self, element: T) -> Self {
        Self {
            element: Some(element.into()),
            ..self
        }
    }

    pub fn set_dispatch_action(self, dispatch_action: bool) -> Self {
        Self {
            dispatch_action: Some(dispatch_action),
            ..self
        }
    }

    pub fn dispatch_action(self) -> Self {
        self.set_dispatch_action(true)
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    pub fn set_hint(self, hint: Text) -> Self {
        Self {
            hint: Some(hint),
            ..self
        }
    }

    pub fn hint<T: Into<String>>(self, hint: T) -> Self {
        self.set_hint(Text::plain(hint))
    }

    pub fn set_optional(self, optional: bool) -> Self {
        Self {
            optional: Some(optional),
            ..self
        }
    }

    pub fn optional(self) -> Self {
        self.set_optional(true)
    }

    pub fn required(self) -> Self {
        self.set_optional(false)
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum InputElement {
    CheckboxGroup(Box<CheckboxGroup>),
    DatePicker(Box<DatePicker>),
    DatetimePicker(Box<DatetimePicker>),
    EmailInput(Box<EmailInput>),
    MultiSelectConversations(Box<MultiSelectConversations>),
    MultiSelectExternals(Box<MultiSelectExternals>),
    MultiSelectPublicChannels(Box<MultiSelectPublicChannels>),
    MultiSelectStaticOptions(Box<MultiSelectStaticOptions>),
    MultiSelectUsers(Box<MultiSelectUsers>),
    NumberInput(Box<NumberInput>),
    PlainTextInput(Box<PlainTextInput>),
    RadioButtonGroup(Box<RadioButtonGroup>),
    SelectConversations(Box<SelectConversations>),
    SelectExternals(Box<SelectExternals>),
    SelectPublicChannels(Box<SelectPublicChannels>),
    SelectStaticOptions(Box<SelectStaticOptions>),
    SelectUsers(Box<SelectUsers>),
    TimePicker(Box<TimePicker>),
    UrlInput(Box<UrlInput>),
}

impl From<CheckboxGroup> for InputElement {
    fn from(value: CheckboxGroup) -> Self {
        Self::CheckboxGroup(Box::new(value))
    }
}

impl From<DatePicker> for InputElement {
    fn from(value: DatePicker) -> Self {
        Self::DatePicker(Box::new(value))
    }
}

impl From<DatetimePicker> for InputElement {
    fn from(value: DatetimePicker) -> Self {
        Self::DatetimePicker(Box::new(value))
    }
}

impl From<EmailInput> for InputElement {
    fn from(value: EmailInput) -> Self {
        Self::EmailInput(Box::new(value))
    }
}

impl From<MultiSelectConversations> for InputElement {
    fn from(value: MultiSelectConversations) -> Self {
        Self::MultiSelectConversations(Box::new(value))
    }
}

impl From<MultiSelectExternals> for InputElement {
    fn from(value: MultiSelectExternals) -> Self {
        Self::MultiSelectExternals(Box::new(value))
    }
}

impl From<MultiSelectPublicChannels> for InputElement {
    fn from(value: MultiSelectPublicChannels) -> Self {
        Self::MultiSelectPublicChannels(Box::new(value))
    }
}

impl From<MultiSelectStaticOptions> for InputElement {
    fn from(value: MultiSelectStaticOptions) -> Self {
        Self::MultiSelectStaticOptions(Box::new(value))
    }
}

impl From<MultiSelectUsers> for InputElement {
    fn from(value: MultiSelectUsers) -> Self {
        Self::MultiSelectUsers(Box::new(value))
    }
}

impl From<NumberInput> for InputElement {
    fn from(value: NumberInput) -> Self {
        Self::NumberInput(Box::new(value))
    }
}

impl From<PlainTextInput> for InputElement {
    fn from(value: PlainTextInput) -> Self {
        Self::PlainTextInput(Box::new(value))
    }
}

impl From<RadioButtonGroup> for InputElement {
    fn from(value: RadioButtonGroup) -> Self {
        Self::RadioButtonGroup(Box::new(value))
    }
}

impl From<SelectConversations> for InputElement {
    fn from(value: SelectConversations) -> Self {
        Self::SelectConversations(Box::new(value))
    }
}

impl From<SelectExternals> for InputElement {
    fn from(value: SelectExternals) -> Self {
        Self::SelectExternals(Box::new(value))
    }
}

impl From<SelectPublicChannels> for InputElement {
    fn from(value: SelectPublicChannels) -> Self {
        Self::SelectPublicChannels(Box::new(value))
    }
}

impl From<SelectStaticOptions> for InputElement {
    fn from(value: SelectStaticOptions) -> Self {
        Self::SelectStaticOptions(Box::new(value))
    }
}

impl From<SelectUsers> for InputElement {
    fn from(value: SelectUsers) -> Self {
        Self::SelectUsers(Box::new(value))
    }
}

impl From<TimePicker> for InputElement {
    fn from(value: TimePicker) -> Self {
        Self::TimePicker(Box::new(value))
    }
}

impl From<UrlInput> for InputElement {
    fn from(value: UrlInput) -> Self {
        Self::UrlInput(Box::new(value))
    }
}
