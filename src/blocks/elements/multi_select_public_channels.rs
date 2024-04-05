use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Multi-select menu Public channels element](https://api.slack.com/reference/block-kit/block-elements#channel_multi_select)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
/// use serde_json::json;
///
/// let menu = MultiSelectPublicChannels::new()
///     .set_action_id("text1234")
///     .placeholder("Select channels");
///
/// let expected = json!({
///     "type": "multi_channels_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select channels",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectPublicChannels {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

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

impl Default for MultiSelectPublicChannels {
    fn default() -> Self {
        Self {
            kind: "multi_channels_select",
            action_id: "".into(),
            initial_channels: vec![],
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl MultiSelectPublicChannels {
    /// Constructs a Multi-select menu Public channels element with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new();
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": ""
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_channels field directly.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .set_initial_channels(
    ///         vec!["channel_0".to_string(), "channel_1".to_string()]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "initial_channels": ["channel_0", "channel_1"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_initial_channels(self, initial_channels: Vec<String>) -> Self {
        Self {
            initial_channels,
            ..self
        }
    }

    /// Adds string to initial_channels field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .push_initial_channel("channel_0");
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "initial_channels": ["channel_0"]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn push_initial_channel<T: Into<String>>(self, channel: T) -> Self {
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

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectPublicChannels, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?",
    ///             "emoji": true
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?",
    ///             "emoji": true
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it",
    ///             "emoji": true
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!",
    ///             "emoji": true
    ///         }
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets max_selected_items field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .set_max_selected_items(3);
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "max_selected_items": 3
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_max_selected_items<T: Into<i64>>(self, items: T) -> Self {
        Self {
            max_selected_items: Some(items.into()),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::MultiSelectPublicChannels;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }

    /// Sets placeholder field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{MultiSelectPublicChannels, Text};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectPublicChannels::new()
    ///     .set_placeholder(Text::plain("Select channels"));
    ///
    /// let expected = json!({
    ///     "type": "multi_channels_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select channels",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }
}
