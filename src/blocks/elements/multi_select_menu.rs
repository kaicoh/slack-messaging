use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::select_menu_types::{
    Conversations, ExternalDataSource, MultiSelectType, PublicChannels, StaticOptions, Users,
};
use serde::Serialize;
use std::marker::PhantomData;

/// The entry point to build any variant of [`MultiSelectMenu`].
///
/// # Example
///
/// ## [Static options](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelect;
/// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
/// let menu = MultiSelect::<StaticOptions>::builder()
///     .action_id("text1234")
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-0"))
///             .value("value-0")
///             .build()
///     )
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-1"))
///             .value("value-1")
///             .build()
///     )
///     .placeholder("Select items")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_static_select",
///     "action_id": "text1234",
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
///     ],
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
///
/// ## [External data source](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelect;
/// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
/// let menu = MultiSelect::<ExternalDataSource>::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder("Select items")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
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
///
/// ## [User list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelect;
/// # use slack_messaging::blocks::elements::select_menu_types::Users;
/// let menu = MultiSelect::<Users>::builder()
///     .action_id("text1234")
///     .initial_user("user9999")
///     .placeholder("Select users")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_users_select",
///     "action_id": "text1234",
///     "initial_users": [
///         "user9999"
///     ],
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select users"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
///
/// ## [Conversation list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelect;
/// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
/// let menu = MultiSelect::<Conversations>::builder()
///     .action_id("text1234")
///     .placeholder("Select conversations")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "multi_conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select conversations"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
///
/// ## [Public channels](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelect;
/// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
/// let menu = MultiSelect::<PublicChannels>::builder()
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
#[derive(Debug, Default, Clone, Serialize)]
pub struct MultiSelect<T>
where
    T: MultiSelectType + Default,
{
    #[serde(serialize_with = "MultiSelectType::serialize")]
    pub(super) r#type: PhantomData<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) option_groups: Vec<OptGroup>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_users: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_conversations: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<ConversationFilter>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) initial_channels: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<PlainText>,
}

/// [Multi-select menu element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element).
/// To build any of this variant, you should use [`MultiSelect`].
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MultiSelectMenu {
    /// [Multi-select menu Static options element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
    /// representation.
    StaticOptions(MultiSelect<StaticOptions>),

    /// [Multi-select menu External data source element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
    /// representation.
    ExternalDataSource(MultiSelect<ExternalDataSource>),

    /// [Multi-select menu User list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
    /// representation.
    Users(MultiSelect<Users>),

    /// [Multi-select menu Conversations list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
    /// representation.
    Conversations(MultiSelect<Conversations>),

    /// [Multi-select menu Public channels element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
    /// representation.
    PublicChannels(MultiSelect<PublicChannels>),
}
