use super::{
    MultiSelectStaticOptions,
    composition_objects::{ConfirmationDialog, Opt, OptGroup, Text},
};

impl MultiSelectStaticOptions {
    /// Construct a [`MultiSelectStaticOptionsBuilder`].
    pub fn builder() -> MultiSelectStaticOptionsBuilder {
        MultiSelectStaticOptionsBuilder::default()
    }
}

/// Builder for [`MultiSelectStaticOptions`] object.
#[derive(Debug, Default)]
pub struct MultiSelectStaticOptionsBuilder {
    action_id: Option<String>,
    options: Vec<Opt>,
    option_groups: Vec<OptGroup>,
    initial_options: Vec<Opt>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<Text>,
}

impl MultiSelectStaticOptionsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "action_id": "text1234",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "action_id": "text1234",
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set options field and remove option_groups field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_options(
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
    ///     "type": "multi_static_select",
    ///     "options": [
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
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self {
            options,
            option_groups: vec![],
            ..self
        }
    }

    /// Add Opt object to options field and remove option_groups field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self {
            options,
            option_groups: vec![],
            ..self
        }
    }

    /// Set option_groups field and removes options field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::{Opt, OptGroup};
    /// let group_0 = OptGroup::builder()
    ///     .label("Group Zero")
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("option-00")
    ///                 .value("value-00")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text("option-01")
    ///                 .value("value-01")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let group_1 = OptGroup::builder()
    ///     .label("Group One")
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("option-10")
    ///                 .value("value-10")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text("option-11")
    ///                 .value("value-11")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_option_groups(vec![group_0, group_1])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "option_groups": [
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group Zero"
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-00"
    ///                     },
    ///                     "value": "value-00"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-01"
    ///                     },
    ///                     "value": "value-01"
    ///                 },
    ///             ]
    ///         },
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group One"
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-10"
    ///                     },
    ///                     "value": "value-10"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-11"
    ///                     },
    ///                     "value": "value-11"
    ///                 },
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_option_groups(self, option_groups: Vec<OptGroup>) -> Self {
        Self {
            options: vec![],
            option_groups,
            ..self
        }
    }

    /// Add OptGroup object to option_groups field and removes options field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::{Opt, OptGroup};
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .option_group(
    ///         OptGroup::builder()
    ///             .label("Group Zero")
    ///             .option(
    ///                 Opt::builder()
    ///                     .text("option-00")
    ///                     .value("value-00")
    ///                     .build()
    ///             )
    ///             .option(
    ///                 Opt::builder()
    ///                     .text("option-01")
    ///                     .value("value-01")
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "option_groups": [
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group Zero"
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-00"
    ///                     },
    ///                     "value": "value-00"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-01"
    ///                     },
    ///                     "value": "value-01"
    ///                 },
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option_group(self, option_group: OptGroup) -> Self {
        let Self {
            mut option_groups, ..
        } = self;
        option_groups.push(option_group);
        Self {
            options: vec![],
            option_groups,
            ..self
        }
    }

    /// Set initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectStaticOptions::builder()
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
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
    ///     "initial_options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
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

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectStaticOptions::builder()
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
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelectStaticOptions::builder()
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
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_max_selected_items(Some(30))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .max_selected_items(30)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// # use slack_messaging::composition_objects::Text;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .set_placeholder(
    ///         Some(Text::builder()
    ///             .plain_text("Select items")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
    /// let menu = MultiSelectStaticOptions::builder()
    ///     .placeholder("Select items")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_static_select",
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

    /// Build a [`MultiSelectStaticOptions`] object.
    pub fn build(self) -> MultiSelectStaticOptions {
        MultiSelectStaticOptions {
            kind: "multi_static_select",
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
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

    /// Get options value.
    pub fn get_options(&self) -> &[Opt] {
        &self.options
    }

    /// Get option_groupss value.
    pub fn get_option_groups(&self) -> &[OptGroup] {
        &self.option_groups
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
