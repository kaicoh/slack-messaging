use super::{
    composition_objects::{ConfirmationDialog, Opt, Text},
    SelectExternals,
};

impl SelectExternals {
    /// Construct a [`SelectExternalsBuilder`].
    pub fn builder() -> SelectExternalsBuilder {
        SelectExternalsBuilder::default()
    }
}

/// Builder for [`SelectExternals`] object.
#[derive(Debug, Default)]
pub struct SelectExternalsBuilder {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_option: Option<Opt>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl SelectExternalsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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

    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = SelectExternals::builder()
    ///     .set_initial_option(
    ///         Some(Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
    ///     "initial_option": {
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "option-0"
    ///        },
    ///        "value": "value-0"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_option(self, initial_option: Option<Opt>) -> Self {
        Self {
            initial_option,
            ..self
        }
    }

    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = SelectExternals::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
    ///     "initial_option": {
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "option-0"
    ///        },
    ///        "value": "value-0"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_option(self, initial_option: Opt) -> Self {
        self.set_initial_option(Some(initial_option))
    }

    /// Set min_query_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .set_min_query_length(Some(3))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
    ///     "min_query_length": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_min_query_length(self, length: Option<i64>) -> Self {
        Self {
            min_query_length: length,
            ..self
        }
    }

    /// Set min_query_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .min_query_length(3)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
    ///     "min_query_length": 3
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn min_query_length(self, length: impl Into<i64>) -> Self {
        self.set_min_query_length(Some(length.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = SelectExternals::builder()
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
    ///     "type": "external_select",
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
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = SelectExternals::builder()
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
    ///     "type": "external_select",
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

    /// Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = SelectExternals::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select an item")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// let menu = SelectExternals::builder()
    ///     .placeholder("Select an item")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "external_select",
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

    /// Build a [`SelectExternals`] object.
    pub fn build(self) -> SelectExternals {
        SelectExternals {
            kind: "external_select",
            action_id: self.action_id,
            min_query_length: self.min_query_length,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get min_query_length value.
    pub fn get_min_query_length(&self) -> &Option<i64> {
        &self.min_query_length
    }

    /// Get initial_option value.
    pub fn get_initial_option(&self) -> &Option<Opt> {
        &self.initial_option
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
