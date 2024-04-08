use super::{
    composition_objects::{ConfirmationDialog, Opt, Text},
    MultiSelectExternals,
};

impl MultiSelectExternals {
    /// Construct a [`MultiSelectExternalsBuilder`].
    pub fn builder() -> MultiSelectExternalsBuilder {
        MultiSelectExternalsBuilder::default()
    }
}

/// Builder for [`MultiSelectExternals`] object.
#[derive(Debug, Default)]
pub struct MultiSelectExternalsBuilder {
    action_id: Option<String>,
    min_query_length: Option<i64>,
    initial_options: Vec<Opt>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl MultiSelectExternalsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
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

    /// Set initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("option-0")
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text("option-1")
    ///                 .value("value-1")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "initial_options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         },
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1"
    ///             },
    ///             "value": "value-1"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_options(self, initial_options: Vec<Opt>) -> Self {
        Self {
            initial_options,
            ..self
        }
    }

    /// Add Opt object to initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectExternals::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text("option-1")
    ///             .value("value-1")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "initial_options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         },
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1"
    ///             },
    ///             "value": "value-1"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_option(self, initial_option: Opt) -> Self {
        let Self {
            mut initial_options,
            ..
        } = self;
        initial_options.push(initial_option);
        Self {
            initial_options,
            ..self
        }
    }

    /// Set min_query_length field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_min_query_length(Some(5))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "min_query_length": 5
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .min_query_length(5)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "min_query_length": 5
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectExternals::builder()
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
    ///     "type": "multi_external_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectExternals::builder()
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
    ///     "type": "multi_external_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_max_selected_items(Some(30))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "max_selected_items": 30
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .max_selected_items(30)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "max_selected_items": 30
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = MultiSelectExternals::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select items")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select items"
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
    /// # use slack_messaging::blocks::elements::MultiSelectExternals;
    /// let menu = MultiSelectExternals::builder()
    ///     .placeholder("Select items")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_external_select",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select items"
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

    /// Build a [`MultiSelectExternals`] object.
    pub fn build(self) -> MultiSelectExternals {
        MultiSelectExternals {
            kind: "multi_external_select",
            action_id: self.action_id,
            min_query_length: self.min_query_length,
            initial_options: self.initial_options,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
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

    /// Get initial_options value.
    pub fn get_initial_options(&self) -> &[Opt] {
        &self.initial_options
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get max_selected_items value.
    pub fn get_max_selected_items(&self) -> &Option<i64> {
        &self.max_selected_items
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
