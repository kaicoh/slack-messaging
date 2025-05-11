use super::{
    SelectPublicChannels,
    composition_objects::{ConfirmationDialog, Text},
};

impl SelectPublicChannels {
    /// Construct a [`SelectPublicChannelsBuilder`].
    pub fn builder() -> SelectPublicChannelsBuilder {
        SelectPublicChannelsBuilder::default()
    }
}

/// Builder for [`SelectPublicChannels`] object.
#[derive(Debug, Default)]
pub struct SelectPublicChannelsBuilder {
    action_id: Option<String>,
    initial_channel: Option<String>,
    confirm: Option<ConfirmationDialog>,
    response_url_enabled: Option<bool>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl SelectPublicChannelsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
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
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
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

    /// Set initial_channel field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_initial_channel(Some("channel_0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "initial_channel": "channel_0"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_channel(self, initial_channel: Option<String>) -> Self {
        Self {
            initial_channel,
            ..self
        }
    }

    /// Set initial_channel field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .initial_channel("channel_0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "initial_channel": "channel_0"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_channel(self, initial_channel: impl Into<String>) -> Self {
        self.set_initial_channel(Some(initial_channel.into()))
    }

    /// Sets confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = SelectPublicChannels::builder()
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
    ///     "type": "channels_select",
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

    /// Sets confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = SelectPublicChannels::builder()
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
    ///     "type": "channels_select",
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

    /// Set response_url_enabled field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_response_url_enabled(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_response_url_enabled(self, enabled: Option<bool>) -> Self {
        Self {
            response_url_enabled: enabled,
            ..self
        }
    }

    /// Set response_url_enabled field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .response_url_enabled(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "response_url_enabled": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn response_url_enabled(self, enabled: bool) -> Self {
        self.set_response_url_enabled(Some(enabled))
    }

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
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
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
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
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select an item")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item"
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
    /// # use slack_messaging::blocks::elements::SelectPublicChannels;
    /// let menu = SelectPublicChannels::builder()
    ///     .placeholder("Select an item")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channels_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item"
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

    /// Build a [`SelectPublicChannels`] object.
    pub fn build(self) -> SelectPublicChannels {
        SelectPublicChannels {
            kind: "channels_select",
            action_id: self.action_id,
            initial_channel: self.initial_channel,
            response_url_enabled: self.response_url_enabled,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get initial_channel value.
    pub fn get_initial_channel(&self) -> &Option<String> {
        &self.initial_channel
    }

    /// Get response_url_enabled value.
    pub fn get_response_url_enabled(&self) -> &Option<bool> {
        &self.response_url_enabled
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get focus_on_load value.
    pub fn get_focus_on_load(&self) -> &Option<bool> {
        &self.focus_on_load
    }

    /// Get action_id value.
    pub fn get_placeholder(&self) -> &Option<Text> {
        &self.placeholder
    }
}
