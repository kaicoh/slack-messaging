use super::elements::{
    CheckboxGroup, DatePicker, DatetimePicker, EmailInput, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    NumberInput, PlainTextInput, RadioButtonGroup, SelectConversations, SelectExternals,
    SelectPublicChannels, SelectStaticOptions, SelectUsers, Text, TimePicker, UrlInput,
};
use crate::plain_text;
use serde::Serialize;

/// [Input block](https://api.slack.com/reference/block-kit/blocks#input)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::Input;
/// use slack_messaging::blocks::elements::PlainTextInput;
/// use serde_json::json;
///
/// let input = Input::new()
///     .set_block_id("input_1")
///     .label("label text")
///     .set_element(
///         PlainTextInput::new()
///             .set_action_id("text_area_1")
///             .multiline()
///             .placeholder("Enter some plain text.")
///     )
///     .optional();
///
/// let expected = json!({
///     "type": "input",
///     "block_id": "input_1",
///     "label": {
///         "type": "plain_text",
///         "text": "label text",
///         "emoji": true
///     },
///     "element": {
///         "type": "plain_text_input",
///         "action_id": "text_area_1",
///         "multiline": true,
///         "placeholder": {
///             "type": "plain_text",
///             "text": "Enter some plain text.",
///             "emoji": true
///         }
///     },
///     "optional": true
/// });
///
/// let input_json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(input_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
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
            label: plain_text!(""),
            element: None,
            dispatch_action: None,
            block_id: None,
            hint: None,
            optional: None,
        }
    }
}

impl Input {
    /// Constructs an Input block.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new();
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets label field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let input = Input::new()
    ///     .set_label(Text::plain("label text"));
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text",
    ///         "emoji": true
    ///     },
    ///     "element": null
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_label(self, label: Text) -> Self {
        Self { label, ..self }
    }

    /// Sets an object to element field. The argument is an any object
    /// that can transform into the enum [InputElement].
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use slack_messaging::blocks::elements::PlainTextInput;
    /// use serde_json::json;
    ///
    /// let input = Input::new()
    ///     .set_element(
    ///         PlainTextInput::new().set_action_id("input_1")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "action_id": "input_1"
    ///     },
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_element<T: Into<InputElement>>(self, element: T) -> Self {
        Self {
            element: Some(element.into()),
            ..self
        }
    }

    /// Sets dispatch_action field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new()
    ///     .set_dispatch_action(true);
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "dispatch_action": true
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_dispatch_action(self, dispatch_action: bool) -> Self {
        Self {
            dispatch_action: Some(dispatch_action),
            ..self
        }
    }

    /// Sets true to dispatch_action field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new().dispatch_action();
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "dispatch_action": true
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn dispatch_action(self) -> Self {
        self.set_dispatch_action(true)
    }

    /// Sets block_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new().set_block_id("input_1");
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "block_id": "input_1"
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    /// Sets hint field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let input = Input::new()
    ///     .set_hint(Text::plain("Some hints for input"));
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "hint": {
    ///         "type": "plain_text",
    ///         "text": "Some hints for input",
    ///         "emoji": true
    ///     },
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_hint(self, hint: Text) -> Self {
        Self {
            hint: Some(hint),
            ..self
        }
    }

    /// Sets optional field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new().set_optional(true);
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "optional": true
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn set_optional(self, optional: bool) -> Self {
        Self {
            optional: Some(optional),
            ..self
        }
    }

    /// Sets true to optional field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new().optional();
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "optional": true
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn optional(self) -> Self {
        self.set_optional(true)
    }

    /// Sets false to optional field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Input;
    /// use serde_json::json;
    ///
    /// let input = Input::new().required();
    ///
    /// let expected = json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "element": null,
    ///     "optional": false
    /// });
    ///
    /// let input_json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(input_json, expected);
    /// ```
    pub fn required(self) -> Self {
        self.set_optional(false)
    }
}

/// Objects that can be an element of the [Input]'s element field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputElement {
    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    CheckboxGroup(Box<CheckboxGroup>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

    /// [Email input element](https://api.slack.com/reference/block-kit/block-elements#email)
    /// representation
    EmailInput(Box<EmailInput>),

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

    /// [Number input element](https://api.slack.com/reference/block-kit/block-elements#number)
    /// representation
    NumberInput(Box<NumberInput>),

    /// [Plain-text input element](https://api.slack.com/reference/block-kit/block-elements#input)
    /// representation
    PlainTextInput(Box<PlainTextInput>),

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

    /// [URL input element](https://api.slack.com/reference/block-kit/block-elements#url)
    /// representation
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
