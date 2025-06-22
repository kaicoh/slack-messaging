use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::{
    ConversationsList, ExternalDataSource, PublicChannels, Select, SelectConversations,
    SelectExternals, SelectMenu, SelectPublicChannels, SelectStaticOptions, SelectUsers,
    StaticOptions, UserList,
};
use std::marker::PhantomData;

macro_rules! impl_select_menu_builder {
    ($($struct:tt using $ty:ty),*) => {
        $(
            impl $struct {
                pub fn builder() -> SelectMenuBuilder<$ty> {
                    SelectMenuBuilder::<$ty>::default()
                }
            }
        )*
    };
}

impl_select_menu_builder! {
    SelectConversations using ConversationsList,
    SelectExternals using ExternalDataSource,
    SelectPublicChannels using PublicChannels,
    SelectStaticOptions using StaticOptions,
    SelectUsers using UserList
}

/// Builder for [`SelectMenu`] object.
#[derive(Debug, Default)]
pub struct SelectMenuBuilder<T> {
    r#type: PhantomData<T>,
    action_id: Option<String>,
    options: Vec<Opt>,
    option_groups: Vec<OptGroup>,
    initial_option: Option<Opt>,
    min_query_length: Option<i64>,
    initial_user: Option<String>,
    initial_conversation: Option<String>,
    default_to_current_conversation: Option<bool>,
    response_url_enabled: Option<bool>,
    filter: Option<ConversationFilter>,
    initial_channel: Option<String>,
    confirm: Option<ConfirmationDialog>,
    focus_on_load: Option<bool>,
    placeholder: Option<PlainText>,
}

impl<T> SelectMenuBuilder<T> {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// let menu = SelectStaticOptions::builder()
    ///     .set_action_id(Some("text1234".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// let menu = SelectStaticOptions::builder()
    ///     .action_id("text1234")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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

    // Set focus_on_load field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
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
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_focus_on_load(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
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
    /// # use slack_messaging::composition_objects::PlainText;
    /// let menu = SelectPublicChannels::builder()
    ///     .set_placeholder(
    ///         Some(PlainText::builder()
    ///             .text("Select an item")
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
    pub fn set_placeholder(self, placeholder: Option<PlainText>) -> Self {
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

    /// Get focus_on_load value.
    pub fn get_focus_on_load(&self) -> &Option<bool> {
        &self.focus_on_load
    }

    /// Get action_id value.
    pub fn get_placeholder(&self) -> &Option<PlainText> {
        &self.placeholder
    }
}

impl SelectMenuBuilder<StaticOptions> {
    /// Set options field and removes option_groups field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = SelectStaticOptions::builder()
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
    ///     "type": "static_select",
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
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = SelectStaticOptions::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
    ///             .value("value-0")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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

    /// Set option_groups field and remove options field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
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
    /// let menu = SelectStaticOptions::builder()
    ///     .set_option_groups(vec![group_0, group_1])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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

    /// Add OptGroup object to option_groups field and remove options field.
    /// (Either options or option_groups field should exist.)
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// # use slack_messaging::composition_objects::{Opt, OptGroup};
    /// # use slack_messaging::plain_text;
    /// let menu = SelectStaticOptions::builder()
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
    ///     "type": "static_select",
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

    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = SelectStaticOptions::builder()
    ///     .set_initial_option(
    ///         Some(Opt::builder()
    ///             .text(plain_text!("option-0"))
    ///             .value("value-0")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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
    /// # use slack_messaging::blocks::elements::SelectStaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = SelectStaticOptions::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
    ///             .value("value-0")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "static_select",
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

    /// Get options value.
    pub fn get_options(&self) -> &[Opt] {
        &self.options
    }

    /// Get option_groupss value.
    pub fn get_option_groups(&self) -> &[OptGroup] {
        &self.option_groups
    }

    /// Get initial_option value.
    pub fn get_initial_option(&self) -> &Option<Opt> {
        &self.initial_option
    }

    /// Build a enum variant SelectMenu::StaticOptions object.
    pub fn build(self) -> SelectMenu {
        SelectMenu::StaticOptions(Select::<StaticOptions> {
            action_id: self.action_id,
            options: self.options,
            option_groups: self.option_groups,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl SelectMenuBuilder<ExternalDataSource> {
    /// Set initial_option field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectExternals;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let menu = SelectExternals::builder()
    ///     .set_initial_option(
    ///         Some(Opt::builder()
    ///             .text(plain_text!("option-0"))
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
    /// # use slack_messaging::plain_text;
    /// let menu = SelectExternals::builder()
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0"))
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

    /// Get min_query_length value.
    pub fn get_min_query_length(&self) -> &Option<i64> {
        &self.min_query_length
    }

    /// Get initial_option value.
    pub fn get_initial_option(&self) -> &Option<Opt> {
        &self.initial_option
    }

    /// Build a enum variant SelectMenu::ExternalDataSource object.
    pub fn build(self) -> SelectMenu {
        SelectMenu::ExternalDataSource(Select::<ExternalDataSource> {
            action_id: self.action_id,
            min_query_length: self.min_query_length,
            initial_option: self.initial_option,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl SelectMenuBuilder<UserList> {
    /// Set initial_user field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectUsers;
    /// let menu = SelectUsers::builder()
    ///     .set_initial_user(Some("user_000".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "users_select",
    ///     "initial_user": "user_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_user(self, initial_user: Option<String>) -> Self {
        Self {
            initial_user,
            ..self
        }
    }

    /// Set initial_user field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectUsers;
    /// let menu = SelectUsers::builder()
    ///     .initial_user("user_000")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "users_select",
    ///     "initial_user": "user_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_user(self, initial_user: impl Into<String>) -> Self {
        self.set_initial_user(Some(initial_user.into()))
    }

    /// Get initial_user value.
    pub fn get_initial_user(&self) -> &Option<String> {
        &self.initial_user
    }

    /// Build a enum variant SelectMenu::UserList object.
    pub fn build(self) -> SelectMenu {
        SelectMenu::UserList(Select::<UserList> {
            action_id: self.action_id,
            initial_user: self.initial_user,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl SelectMenuBuilder<ConversationsList> {
    /// Set initial_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_initial_conversation(Some("conversation_000".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "initial_conversation": "conversation_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_initial_conversation(self, value: Option<String>) -> Self {
        Self {
            initial_conversation: value,
            ..self
        }
    }

    /// Set initial_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .initial_conversation("conversation_000")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "initial_conversation": "conversation_000"
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn initial_conversation(self, value: impl Into<String>) -> Self {
        self.set_initial_conversation(Some(value.into()))
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_default_to_current_conversation(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_default_to_current_conversation(self, value: Option<bool>) -> Self {
        Self {
            default_to_current_conversation: value,
            ..self
        }
    }

    /// Set default_to_current_conversation field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .default_to_current_conversation(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
    ///     "default_to_current_conversation": true
    /// });
    ///
    /// let json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn default_to_current_conversation(self, value: bool) -> Self {
        self.set_default_to_current_conversation(Some(value))
    }

    /// Set response_url_enabled field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .set_response_url_enabled(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
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
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// let menu = SelectConversations::builder()
    ///     .response_url_enabled(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "conversations_select",
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

    /// Set filter field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = SelectConversations::builder()
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
    ///     "type": "conversations_select",
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
    /// # use slack_messaging::blocks::elements::SelectConversations;
    /// # use slack_messaging::composition_objects::{ConversationFilter, Conversation};
    /// let menu = SelectConversations::builder()
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
    ///     "type": "conversations_select",
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

    /// Get initial_conversation value.
    pub fn get_initial_conversation(&self) -> &Option<String> {
        &self.initial_conversation
    }

    /// Get default_to_current_conversation value.
    pub fn get_default_to_current_conversation(&self) -> &Option<bool> {
        &self.default_to_current_conversation
    }

    /// Get response_url_enabled value.
    pub fn get_response_url_enabled(&self) -> &Option<bool> {
        &self.response_url_enabled
    }

    /// Get filter value.
    pub fn get_filter(&self) -> &Option<ConversationFilter> {
        &self.filter
    }

    /// Build a enum variant SelectMenu::ConversationsList object.
    pub fn build(self) -> SelectMenu {
        SelectMenu::ConversationsList(Select::<ConversationsList> {
            action_id: self.action_id,
            initial_conversation: self.initial_conversation,
            default_to_current_conversation: self.default_to_current_conversation,
            response_url_enabled: self.response_url_enabled,
            filter: self.filter,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}

impl SelectMenuBuilder<PublicChannels> {
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

    /// Get initial_channel value.
    pub fn get_initial_channel(&self) -> &Option<String> {
        &self.initial_channel
    }

    /// Get response_url_enabled value.
    pub fn get_response_url_enabled(&self) -> &Option<bool> {
        &self.response_url_enabled
    }

    /// Build a enum variant SelectMenu::PublicChannels object.
    pub fn build(self) -> SelectMenu {
        SelectMenu::PublicChannels(Select::<PublicChannels> {
            action_id: self.action_id,
            initial_channel: self.initial_channel,
            response_url_enabled: self.response_url_enabled,
            confirm: self.confirm,
            focus_on_load: self.focus_on_load,
            placeholder: self.placeholder,
            ..Default::default()
        })
    }
}
