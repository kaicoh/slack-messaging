use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Multi-select menu Public channels element](https://api.slack.com/reference/block-kit/block-elements#channel_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
/// let menu = MultiSelectPublicChannels::builder()
///     .action_id("text1234")
///     .placeholder("Select channels")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_channels_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select channels"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectPublicChannels {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_channels: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl MultiSelectPublicChannels {
    /// Construct a [`MultiSelectPublicChannelsBuilder`].
    pub fn builder() -> MultiSelectPublicChannelsBuilder {
        MultiSelectPublicChannelsBuilder::default()
    }
}

/// Builder for [`MultiSelectPublicChannels`] object.
#[derive(Debug, Default)]
pub struct MultiSelectPublicChannelsBuilder {
    action_id: Option<String>,
    initial_channels: Vec<String>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl MultiSelectPublicChannelsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set initial_channels field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_initial_channels(
    ///         vec!["channel_0".to_string(), "channel_1".to_string()]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "initial_channels": ["channel_0", "channel_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_channels(self, initial_channels: Vec<String>) -> Self {
        Self {
            initial_channels,
            ..self
        }
    }

    /// Add channel id to initial_channels field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .initial_channel("channel_0")
    ///     .initial_channel("channel_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "initial_channels": ["channel_0", "channel_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_channel(self, channel: impl Into<String>) -> Self {
        let Self {
            mut initial_channels,
            ..
        } = self;
        initial_channels.push(channel.into());
        Self {
            initial_channels,
            ..self
        }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{MultiSelectPublicChannels, ConfirmationDialog};
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_confirm(
    ///         Some(ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{MultiSelectPublicChannels, ConfirmationDialog};
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set max_selected_items field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_max_selected_items(Some(3))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_selected_items(self, items: Option<i64>) -> Self {
        Self {
            max_selected_items: items,
            ..self
        }
    }

    /// Set max_selected_items field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .max_selected_items(3)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_selected_items(self, items: impl Into<i64>) -> Self {
        self.set_max_selected_items(Some(items.into()))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: Option<bool>) -> Self {
        Self {
            focus_on_load,
            ..self
        }
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .set_placeholder(Some(plain_text!("Select channels")))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select channels"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Option<Text>) -> Self {
        Self {
            placeholder,
            ..self
        }
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// let menu = MultiSelectPublicChannels::builder()
    ///     .placeholder("Select channels")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_channels_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select channels"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn placeholder(self, placeholder: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(placeholder.into()).build();
        self.set_placeholder(Some(text))
    }

    /// Build a [`MultiSelectPublicChannels`] object.
    pub fn build(self) -> MultiSelectPublicChannels {
        MultiSelectPublicChannels {
            kind: "multi_channels_select",
            action_id: self.action_id,
            initial_channels: self.initial_channels,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }
}
