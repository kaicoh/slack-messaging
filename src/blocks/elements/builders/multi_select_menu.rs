use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::select_menu_types::{
    Conversations, ExternalDataSource, PublicChannels, StaticOptions, Users,
};
use super::{MultiSelect, MultiSelectMenu};
use std::marker::PhantomData;

macro_rules! impl_multi_select_menu_builder {
    ($($ty:ty),*) => {
        $(
            impl MultiSelect<$ty> {
                pub fn builder() -> MultiSelectMenuBuilder<$ty> {
                    MultiSelectMenuBuilder::<$ty>::default()
                }
            }
        )*
    };
}

impl_multi_select_menu_builder! {
    Conversations,
    ExternalDataSource,
    PublicChannels,
    StaticOptions,
    Users
}

/// Builder for [`MultiSelectMenu`] object.
#[derive(Debug, Default)]
pub struct MultiSelectMenuBuilder<T> {
    r#type: PhantomData<T>,
    action_id: Option<String>,
    options: Vec<Opt>,
    option_groups: Vec<OptGroup>,
    initial_options: Vec<Opt>,
    min_query_length: Option<i64>,
    initial_users: Vec<String>,
    initial_conversations: Vec<String>,
    default_to_current_conversation: Option<bool>,
    filter: Option<ConversationFilter>,
    initial_channels: Vec<String>,
    confirm: Option<ConfirmationDialog>,
    max_selected_items: Option<i64>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
}

impl<T> MultiSelectMenuBuilder<T> {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// let menu = MultiSelect::<StaticOptions>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// let menu = MultiSelect::<StaticOptions>::builder()
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

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Users;
    /// let menu = MultiSelect::<Users>::builder()
    ///     .set_max_selected_items(Some(30))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Users;
    /// let menu = MultiSelect::<Users>::builder()
    ///     .max_selected_items(30)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .focus_on_load(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let menu = MultiSelect::<PublicChannels>::builder()
    ///     .set_placeholder(
    ///         Some(PlainText::builder()
    ///             .text("Select channels")
    ///             .build())
    ///     )
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
    pub fn set_placeholder(self, placeholder: Option<PlainText>) -> Self {
        Self {
            placeholder,
            ..self
        }
    }

    /// Set placeholder field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
    /// let menu = MultiSelect::<PublicChannels>::builder()
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
        let text = PlainText::builder().text(placeholder).build();
        self.set_placeholder(Some(text))
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
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
    pub fn get_placeholder(&self) -> &Option<PlainText> {
        &self.placeholder
    }
}

impl MultiSelectMenuBuilder<StaticOptions> {
    /// Set options field and remove option_groups field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<StaticOptions>::builder()
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(plain_text!("option-0"))
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text(plain_text!("option-1"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<StaticOptions>::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::{Opt, OptGroup};
    /// # use slack_messaging::plain_text;
    /// let group_0 = OptGroup::builder()
    ///     .label("Group Zero")
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(plain_text!("option-00"))
    ///                 .value("value-00")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text(plain_text!("option-01"))
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
    ///                 .text(plain_text!("option-10"))
    ///                 .value("value-10")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text(plain_text!("option-11"))
    ///                 .value("value-11")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let menu = MultiSelect::<StaticOptions>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::{Opt, OptGroup};
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<StaticOptions>::builder()
    ///     .option_group(
    ///         OptGroup::builder()
    ///             .label("Group Zero")
    ///             .option(
    ///                 Opt::builder()
    ///                     .text(plain_text!("option-00"))
    ///                     .value("value-00")
    ///                     .build()
    ///             )
    ///             .option(
    ///                 Opt::builder()
    ///                     .text(plain_text!("option-01"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<StaticOptions>::builder()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(plain_text!("option-0"))
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text(plain_text!("option-1"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<StaticOptions>::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
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

    /// Build a enum variant MultiSelectMenu::StaticOptions object.
    pub fn build(self) -> MultiSelectMenu {
        MultiSelectMenu::StaticOptions(MultiSelect::<StaticOptions> {
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_options: self.initial_options,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
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
}

impl MultiSelectMenuBuilder<ExternalDataSource> {
    /// Set initial_options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(plain_text!("option-0"))
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text(plain_text!("option-1"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-1"))
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
    /// let menu = MultiSelect::<ExternalDataSource>::builder()
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

    /// Get min_query_length value.
    pub fn get_min_query_length(&self) -> &Option<i64> {
        &self.min_query_length
    }

    /// Get initial_options value.
    pub fn get_initial_options(&self) -> &[Opt] {
        &self.initial_options
    }

    /// Build a enum variant MultiSelectMenu::ExternalDataSource object.
    pub fn build(self) -> MultiSelectMenu {
        MultiSelectMenu::ExternalDataSource(MultiSelect::<ExternalDataSource> {
            action_id: self.action_id,
            initial_options: self.initial_options,
            min_query_length: self.min_query_length,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl MultiSelectMenuBuilder<Users> {
    /// Set initial_users field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Users;
    /// let menu = MultiSelect::<Users>::builder()
    ///     .set_initial_users(
    ///         vec!["user0000".into(), "user9999".into()]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "initial_users": ["user0000", "user9999"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_users(self, initial_users: Vec<String>) -> Self {
        Self {
            initial_users,
            ..self
        }
    }

    /// Add user id to initial_users field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Users;
    /// let menu = MultiSelect::<Users>::builder()
    ///     .initial_user("user0000")
    ///     .initial_user("user0001")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_users_select",
    ///     "initial_users": ["user0000", "user0001"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_user(self, user: impl Into<String>) -> Self {
        let Self {
            mut initial_users, ..
        } = self;
        initial_users.push(user.into());
        Self {
            initial_users,
            ..self
        }
    }

    /// Get initial_users value.
    pub fn get_initial_users(&self) -> &[String] {
        &self.initial_users
    }

    /// Build a enum variant MultiSelectMenu::Users object.
    pub fn build(self) -> MultiSelectMenu {
        MultiSelectMenu::Users(MultiSelect::<Users> {
            action_id: self.action_id,
            initial_users: self.initial_users,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl MultiSelectMenuBuilder<Conversations> {
    /// Set initial_conversations field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .set_initial_conversations(
    ///         vec!["conversation_0".to_string(), "conversation_1".to_string()]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "initial_conversations": ["conversation_0", "conversation_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_conversations(self, initial_conversations: Vec<String>) -> Self {
        Self {
            initial_conversations,
            ..self
        }
    }

    /// Add conversation id to initial_conversations field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .initial_conversation("conversation_0")
    ///     .initial_conversation("conversation_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "initial_conversations": ["conversation_0", "conversation_1"]
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_conversation(self, conversation: impl Into<String>) -> Self {
        let Self {
            mut initial_conversations,
            ..
        } = self;
        initial_conversations.push(conversation.into());
        Self {
            initial_conversations,
            ..self
        }
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .set_default_to_current_conversation(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_default_to_current_conversation(self, current_conversation: Option<bool>) -> Self {
        Self {
            default_to_current_conversation: current_conversation,
            ..self
        }
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .default_to_current_conversation(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn default_to_current_conversation(self, current_conversation: bool) -> Self {
        self.set_default_to_current_conversation(Some(current_conversation))
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .set_filter(
    ///         Some(ConversationFilter::builder()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "filter": {
    ///         "include": [
    ///             "public",
    ///             "mpim"
    ///         ],
    ///         "exclude_bot_users": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_filter(self, filter: Option<ConversationFilter>) -> Self {
        Self { filter, ..self }
    }

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = MultiSelect::<Conversations>::builder()
    ///     .filter(
    ///         ConversationFilter::builder()
    ///             .include(Conversation::Public)
    ///             .include(Conversation::Mpim)
    ///             .exclude_bot_users(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "multi_conversations_select",
    ///     "filter": {
    ///         "include": [
    ///             "public",
    ///             "mpim"
    ///         ],
    ///         "exclude_bot_users": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn filter(self, filter: ConversationFilter) -> Self {
        self.set_filter(Some(filter))
    }

    /// Get initial_conversations value.
    pub fn get_initial_conversations(&self) -> &[String] {
        &self.initial_conversations
    }

    /// Get default_to_current_conversation value.
    pub fn get_default_to_current_conversation(&self) -> &Option<bool> {
        &self.default_to_current_conversation
    }

    /// Get filter value.
    pub fn get_filter(&self) -> &Option<ConversationFilter> {
        &self.filter
    }

    /// Build a enum variant MultiSelectMenu::Conversations object.
    pub fn build(self) -> MultiSelectMenu {
        MultiSelectMenu::Conversations(MultiSelect::<Conversations> {
            action_id: self.action_id,
            initial_conversations: self.initial_conversations,
            default_to_current_conversation: self.default_to_current_conversation,
            filter: self.filter,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl MultiSelectMenuBuilder<PublicChannels> {
    /// Set initial_channels field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
    /// let menu = MultiSelect::<PublicChannels>::builder()
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
    /// # use slack_messaging::blocks::elements::MultiSelect;
    /// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
    /// let menu = MultiSelect::<PublicChannels>::builder()
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

    /// Get initial_channels value.
    pub fn get_initial_channels(&self) -> &[String] {
        &self.initial_channels
    }

    /// Build a enum variant MultiSelectMenu::PublicChannels object.
    pub fn build(self) -> MultiSelectMenu {
        MultiSelectMenu::PublicChannels(MultiSelect::<PublicChannels> {
            action_id: self.action_id,
            initial_channels: self.initial_channels,
            confirm: self.confirm,
            max_selected_items: self.max_selected_items,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}
