use super::elements::{
    Checkboxes, DatePicker, DatetimePicker, EmailInput, FileInput, MultiSelectConversations,
    MultiSelectExternals, MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
    NumberInput, PlainTextInput, RadioButtonGroup, RichTextInput, SelectConversations,
    SelectExternals, SelectPublicChannels, SelectStaticOptions, SelectUsers, Text, TimePicker,
    UrlInput,
};
use serde::Serialize;

/// [Input block](https://api.slack.com/reference/block-kit/blocks#input)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Input;
/// # use slack_messaging::blocks::elements::PlainTextInput;
/// let input = Input::builder()
///     .block_id("input_1")
///     .label("label text")
///     .element(
///         PlainTextInput::builder()
///             .action_id("text_area_1")
///             .multiline(true)
///             .placeholder("Enter some plain text.")
///             .build()
///     )
///     .optional(true)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "input",
///     "block_id": "input_1",
///     "label": {
///         "type": "plain_text",
///         "text": "label text"
///     },
///     "element": {
///         "type": "plain_text_input",
///         "action_id": "text_area_1",
///         "multiline": true,
///         "placeholder": {
///             "type": "plain_text",
///             "text": "Enter some plain text."
///         }
///     },
///     "optional": true
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Input {
    #[serde(rename = "type")]
    kind: &'static str,

    label: Text,

    element: InputElement,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    hint: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    optional: Option<bool>,
}

impl Input {
    /// Construct an [`InputBuilder`].
    pub fn builder() -> InputBuilder {
        InputBuilder::default()
    }
}

/// Builder for [`Input`] object.
#[derive(Debug, Default)]
pub struct InputBuilder {
    label: Option<Text>,
    element: Option<InputElement>,
    dispatch_action: Option<bool>,
    block_id: Option<String>,
    hint: Option<Text>,
    optional: Option<bool>,
}

impl InputBuilder {
    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// let input = Input::builder()
    ///     .set_label(
    ///         Some(Text::builder()
    ///             .plain_text("label text")
    ///             .build())
    ///     )
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_label(self, label: Option<Text>) -> Self {
        Self { label, ..self }
    }

    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn label(self, label: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(label).build();
        self.set_label(Some(text))
    }

    /// Set element field. The argument is an any object
    /// that can transform into the enum [InputElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .set_element(
    ///         Some(PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///             .into())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_element(self, element: Option<InputElement>) -> Self {
        Self { element, ..self }
    }

    /// Set element field. The argument is an any object
    /// that can transform into the enum [InputElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<InputElement>) -> Self {
        self.set_element(Some(element.into()))
    }

    /// Set dispatch_action field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .set_dispatch_action(Some(true))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "dispatch_action": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_dispatch_action(self, dispatch_action: Option<bool>) -> Self {
        Self {
            dispatch_action,
            ..self
        }
    }

    /// Set dispatch_action field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .dispatch_action(true)
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "dispatch_action": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action(self, dispatch_action: bool) -> Self {
        self.set_dispatch_action(Some(dispatch_action))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .set_block_id(Some("input_1".into()))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "block_id": "input_1"
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .block_id("input_1")
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "block_id": "input_1"
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set hint field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// let input = Input::builder()
    ///     .set_hint(
    ///         Some(Text::builder()
    ///             .plain_text("Some hints for input")
    ///             .build())
    ///     )
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "hint": {
    ///         "type": "plain_text",
    ///         "text": "Some hints for input"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_hint(self, hint: Option<Text>) -> Self {
        Self { hint, ..self }
    }

    /// Set hint field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// let input = Input::builder()
    ///     .hint("Some hints for input")
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "hint": {
    ///         "type": "plain_text",
    ///         "text": "Some hints for input"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn hint(self, hint: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(hint).build();
        self.set_hint(Some(text))
    }

    /// Set optional field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// let input = Input::builder()
    ///     .set_optional(Some(true))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "optional": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_optional(self, optional: Option<bool>) -> Self {
        Self { optional, ..self }
    }

    /// Set optional field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::{PlainTextInput, Text};
    /// let input = Input::builder()
    ///     .optional(true)
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "optional": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn optional(self, optional: bool) -> Self {
        self.set_optional(Some(optional))
    }

    /// Build an [`Input`] object. This method will panic if label and element are not set.
    pub fn build(self) -> Input {
        Input {
            kind: "input",
            label: self.label.expect("label must be set to InputBuilder"),
            element: self.element.expect("element must be set to InputBuilder"),
            dispatch_action: self.dispatch_action,
            block_id: self.block_id,
            hint: self.hint,
            optional: self.optional,
        }
    }
}

/// Objects that can be an element of the [Input]'s element field.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputElement {
    /// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
    /// representation
    Checkboxes(Box<Checkboxes>),

    /// [Date picker element](https://api.slack.com/reference/block-kit/block-elements#datepicker)
    /// representation
    DatePicker(Box<DatePicker>),

    /// [Datetime picker element](https://api.slack.com/reference/block-kit/block-elements#datetimepicker)
    /// representation
    DatetimePicker(Box<DatetimePicker>),

    /// [Email input element](https://api.slack.com/reference/block-kit/block-elements#email)
    /// representation
    EmailInput(Box<EmailInput>),

    /// [File input element](https://api.slack.com/reference/block-kit/block-elements#file_input)
    /// representation
    FileInput(Box<FileInput>),

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

    /// [Rich text input element](https://api.slack.com/reference/block-kit/block-elements#rich_text_input)
    /// representation
    RichTextInput(Box<RichTextInput>),

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

macro_rules! input_from {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for InputElement {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

input_from! {
    Checkboxes,
    DatePicker,
    DatetimePicker,
    EmailInput,
    FileInput,
    MultiSelectConversations,
    MultiSelectExternals,
    MultiSelectPublicChannels,
    MultiSelectStaticOptions,
    MultiSelectUsers,
    NumberInput,
    PlainTextInput,
    RadioButtonGroup,
    RichTextInput,
    SelectConversations,
    SelectExternals,
    SelectPublicChannels,
    SelectStaticOptions,
    SelectUsers,
    TimePicker,
    UrlInput
}
