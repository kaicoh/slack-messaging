use serde::Serialize;
use super::elements::{
    Button, CheckboxGroup, DatePicker, DatetimePicker, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions,
    MultiSelectUsers, OverflowMenu, RadioButtonGroup, SelectConversations,
    SelectExternals, SelectPublicChannels, SelectStaticOptions, SelectUsers,
    Text, TimePicker,
};

#[derive(Debug, Serialize)]
pub struct Section {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessory: Option<Accessory>,
}

impl Default for Section {
    fn default() -> Self {
        Self {
            kind: "section",
            text: None,
            block_id: None,
            fields: vec![],
            accessory: None,
        }
    }
}

impl Section {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_text(self, text: Text) -> Self {
        Self {
            text: Some(text),
            ..self
        }
    }

    pub fn plain_text<T: Into<String>>(self, text: T) -> Self {
        self.set_text(Text::plain(text))
    }

    pub fn mrkdwn<T: Into<String>>(self, text: T) -> Self {
        self.set_text(Text::mrkdwn(text))
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    pub fn set_fields(self, fields: Vec<Text>) -> Self {
        Self { fields, ..self }
    }

    pub fn push_field(self, field: Text) -> Self {
        let mut fields = self.fields;
        fields.push(field);
        Self {
            fields,
            ..self
        }
    }

    pub fn push_field_plain<T: Into<String>>(self, field: T) -> Self {
        self.push_field(Text::plain(field))
    }

    pub fn push_field_mrkdwn<T: Into<String>>(self, field: T) -> Self {
        self.push_field(Text::mrkdwn(field))
    }

    pub fn set_accessory<T: Into<Accessory>>(self, accessory: T) -> Self {
        Self {
            accessory: Some(accessory.into()),
            ..self
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum Accessory {
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

impl From<Button> for Accessory {
    fn from(value: Button) -> Self {
        Self::Button(Box::new(value))
    }
}

impl From<CheckboxGroup> for Accessory {
    fn from(value: CheckboxGroup) -> Self {
        Self::CheckboxGroup(Box::new(value))
    }
}

impl From<DatePicker> for Accessory {
    fn from(value: DatePicker) -> Self {
        Self::DatePicker(Box::new(value))
    }
}

impl From<DatetimePicker> for Accessory {
    fn from(value: DatetimePicker) -> Self {
        Self::DatetimePicker(Box::new(value))
    }
}

impl From<MultiSelectConversations> for Accessory {
    fn from(value: MultiSelectConversations) -> Self {
        Self::MultiSelectConversations(Box::new(value))
    }
}

impl From<MultiSelectExternals> for Accessory {
    fn from(value: MultiSelectExternals) -> Self {
        Self::MultiSelectExternals(Box::new(value))
    }
}

impl From<MultiSelectPublicChannels> for Accessory {
    fn from(value: MultiSelectPublicChannels) -> Self {
        Self::MultiSelectPublicChannels(Box::new(value))
    }
}

impl From<MultiSelectStaticOptions> for Accessory {
    fn from(value: MultiSelectStaticOptions) -> Self {
        Self::MultiSelectStaticOptions(Box::new(value))
    }
}

impl From<MultiSelectUsers> for Accessory {
    fn from(value: MultiSelectUsers) -> Self {
        Self::MultiSelectUsers(Box::new(value))
    }
}

impl From<OverflowMenu> for Accessory {
    fn from(value: OverflowMenu) -> Self {
        Self::OverflowMenu(Box::new(value))
    }
}

impl From<RadioButtonGroup> for Accessory {
    fn from(value: RadioButtonGroup) -> Self {
        Self::RadioButtonGroup(Box::new(value))
    }
}

impl From<SelectConversations> for Accessory {
    fn from(value: SelectConversations) -> Self {
        Self::SelectConversations(Box::new(value))
    }
}

impl From<SelectExternals> for Accessory {
    fn from(value: SelectExternals) -> Self {
        Self::SelectExternals(Box::new(value))
    }
}

impl From<SelectPublicChannels> for Accessory {
    fn from(value: SelectPublicChannels) -> Self {
        Self::SelectPublicChannels(Box::new(value))
    }
}

impl From<SelectStaticOptions> for Accessory {
    fn from(value: SelectStaticOptions) -> Self {
        Self::SelectStaticOptions(Box::new(value))
    }
}

impl From<SelectUsers> for Accessory {
    fn from(value: SelectUsers) -> Self {
        Self::SelectUsers(Box::new(value))
    }
}

impl From<TimePicker> for Accessory {
    fn from(value: TimePicker) -> Self {
        Self::TimePicker(Box::new(value))
    }
}
