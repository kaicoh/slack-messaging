use super::elements::{
    Button, CheckboxGroup, DatePicker, DatetimePicker, Image, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    OverflowMenu, RadioButtonGroup, SelectConversations, SelectExternals, SelectPublicChannels,
    SelectStaticOptions, SelectUsers, Text, TimePicker,
};
use serde::Serialize;

/// [Section block](https://api.slack.com/reference/block-kit/blocks#section)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::Section;
/// use slack_messaging::blocks::elements::Image;
/// use serde_json::json;
///
/// let section = Section::new()
///     .set_block_id("section_1")
///     .set_text_mrkdwn("A message *with some bold text* and _some italicized text_.")
///     .push_field_mrkdwn("High")
///     .push_field_plain("String")
///     .set_accessory(
///         Image::new()
///             .set_image_url("http://placekitten.com/700/500")
///             .set_alt_text("Multiple cute kittens")
///     );
///
/// let expected = json!({
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
///             "text": "String",
///             "emoji": true
///         }
///     ],
///     "accessory": {
///         "type": "image",
///         "image_url": "http://placekitten.com/700/500",
///         "alt_text": "Multiple cute kittens"
///     }
/// });
///
/// let section_json = serde_json::to_value(section).unwrap();
///
/// assert_eq!(section_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
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
    /// Constructs a Section block.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use serde_json::json;
    ///
    /// let section = Section::new();
    ///
    /// let expected = json!({
    ///     "type": "section",
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets text field.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_text(Text::mrkdwn("A message *with some bold text* and _some italicized text_."));
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_text(self, text: Text) -> Self {
        Self {
            text: Some(text),
            ..self
        }
    }

    /// Sets text field as plain text object.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_text_plain("A message *with some bold text* and _some italicized text_.");
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "A message *with some bold text* and _some italicized text_.",
    ///         "emoji": true
    ///     },
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_text_plain<T: Into<String>>(self, text: T) -> Self {
        self.set_text(Text::plain(text))
    }

    /// Sets text field as mrkdwn text object.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_text_mrkdwn("A message *with some bold text* and _some italicized text_.");
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_text_mrkdwn<T: Into<String>>(self, text: T) -> Self {
        self.set_text(Text::mrkdwn(text))
    }

    /// Sets block_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_block_id("section_1");
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "block_id": "section_1"
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    /// Sets fields field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_fields(
    ///         vec![
    ///             Text::plain("hello"),
    ///             Text::mrkdwn("*world*")
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "plain_text",
    ///             "text": "hello",
    ///             "emoji": true
    ///         },
    ///         {
    ///             "type": "mrkdwn",
    ///             "text": "*world*"
    ///         },
    ///     ]
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_fields(self, fields: Vec<Text>) -> Self {
        Self { fields, ..self }
    }

    /// Adds Text object to fields field.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .push_field(
    ///         Text::plain("hello world")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "plain_text",
    ///             "text": "hello world",
    ///             "emoji": true
    ///         }
    ///     ]
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn push_field(self, field: Text) -> Self {
        let mut fields = self.fields;
        fields.push(field);
        Self { fields, ..self }
    }

    /// Adds plain_text Text object to fields field.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .push_field_plain("hello world");
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "plain_text",
    ///             "text": "hello world",
    ///             "emoji": true
    ///         }
    ///     ]
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn push_field_plain<T: Into<String>>(self, field: T) -> Self {
        self.push_field(Text::plain(field))
    }

    /// Adds mrkdwn Text object to fields field.
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .push_field_mrkdwn("hello world");
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "mrkdwn",
    ///             "text": "hello world"
    ///         }
    ///     ]
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn push_field_mrkdwn<T: Into<String>>(self, field: T) -> Self {
        self.push_field(Text::mrkdwn(field))
    }

    /// Sets object to accessory field. The argument is an any object
    /// that can transform into the enum [Accessory].
    ///
    /// ```
    /// use slack_messaging::blocks::Section;
    /// use slack_messaging::blocks::elements::Button;
    /// use serde_json::json;
    ///
    /// let section = Section::new()
    ///     .set_accessory(
    ///         Button::new()
    ///             .text("Click Me")
    ///             .set_action_id("button-0")
    ///             .set_value("click_me_123")
    ///             .set_primary()
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "section",
    ///     "accessory": {
    ///         "type": "button",
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Click Me",
    ///             "emoji": true
    ///         },
    ///         "value": "click_me_123",
    ///         "action_id": "button-0",
    ///         "style": "primary"
    ///     }
    /// });
    ///
    /// let section_json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(section_json, expected);
    /// ```
    pub fn set_accessory<T: Into<Accessory>>(self, accessory: T) -> Self {
        Self {
            accessory: Some(accessory.into()),
            ..self
        }
    }
}

/// Objects that can be set to [Section] as an accessory.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Accessory {
    /// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
    /// representation
    Button(Box<Button>),

    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    CheckboxGroup(Box<CheckboxGroup>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

    /// [Image element](https://api.slack.com/reference/block-kit/block-elements#image)
    /// representation
    Image(Box<Image>),

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

impl From<Image> for Accessory {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
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
