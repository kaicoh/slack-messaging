use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::{ConversationsList, ExternalDataSource, PublicChannels, StaticOptions, UserList};
use serde::{Serialize, Serializer};
use std::marker::PhantomData;

/// [Multi-select menu Conversations list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectConversations;
/// let menu = MultiSelectConversations::builder()
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
#[derive(Debug, Copy, Clone)]
pub struct MultiSelectConversations;

/// [Multi-select menu External data source element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectExternals;
/// let menu = MultiSelectExternals::builder()
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
#[derive(Debug, Copy, Clone)]
pub struct MultiSelectExternals;

/// [Multi-select menu Public channels element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
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
#[derive(Debug, Copy, Clone)]
pub struct MultiSelectPublicChannels;

/// [Multi-select menu Static options element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectStaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
/// let menu = MultiSelectStaticOptions::builder()
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
#[derive(Debug, Copy, Clone)]
pub struct MultiSelectStaticOptions;

/// [Multi-select menu User list element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::MultiSelectUsers;
/// let menu = MultiSelectUsers::builder()
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
#[derive(Debug, Copy, Clone)]
pub struct MultiSelectUsers;

pub trait MultiSelectType {
    fn serialize<S>(ty: &PhantomData<Self>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! impl_multi_select_type {
    ($($ty:tt as $expr:tt),*) => {
        $(
            impl MultiSelectType for $ty {
                fn serialize<S>(_: &PhantomData<$ty>, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_str($expr)
                }
            }
        )*
    };
}

impl_multi_select_type! {
    StaticOptions as "multi_static_select",
    ExternalDataSource as "multi_external_select",
    UserList as "multi_users_select",
    ConversationsList as "multi_conversations_select",
    PublicChannels as "multi_channels_select"
}

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

/// [Multi-select menu element](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element)
/// which can be one of the [Static options](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select),
/// [External data source](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select),
/// [User list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select),
/// [Conversations list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select),
/// or [Public channels](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MultiSelectMenu {
    StaticOptions(MultiSelect<StaticOptions>),
    ExternalDataSource(MultiSelect<ExternalDataSource>),
    UserList(MultiSelect<UserList>),
    ConversationsList(MultiSelect<ConversationsList>),
    PublicChannels(MultiSelect<PublicChannels>),
}
