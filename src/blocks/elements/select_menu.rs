use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::{ConversationsList, ExternalDataSource, PublicChannels, StaticOptions, UserList};
use serde::{Serialize, Serializer};
use std::marker::PhantomData;

/// [Select menu of conversations element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectConversations;
/// let menu = SelectConversations::builder()
///     .action_id("text1234")
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "conversations_select",
///     "action_id": "text1234",
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
#[derive(Debug, Copy, Clone)]
pub struct SelectConversations;

/// [Select menu of external data source element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectExternals;
/// let menu = SelectExternals::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
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
#[derive(Debug, Copy, Clone)]
pub struct SelectExternals;

/// [Select menu of public channels element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectPublicChannels;
/// let menu = SelectPublicChannels::builder()
///     .action_id("text1234")
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "channels_select",
///     "action_id": "text1234",
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
#[derive(Debug, Copy, Clone)]
pub struct SelectPublicChannels;

/// [Select menu of static options element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectStaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
/// let menu = SelectStaticOptions::builder()
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
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "static_select",
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
///         "text": "Select an item"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct SelectStaticOptions;

/// [Select menu of users element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::SelectUsers;
/// let menu = SelectUsers::builder()
///     .action_id("text1234")
///     .placeholder("Select an item")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "users_select",
///     "action_id": "text1234",
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
#[derive(Debug, Copy, Clone)]
pub struct SelectUsers;

pub trait SelectType {
    fn serialize<S>(ty: &PhantomData<Self>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer;
}

macro_rules! impl_select_type {
    ($($ty:tt as $expr:tt),*) => {
        $(
            impl SelectType for $ty {
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

impl_select_type! {
    StaticOptions as "static_select",
    ExternalDataSource as "external_select",
    UserList as "users_select",
    ConversationsList as "conversations_select",
    PublicChannels as "channels_select"
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Select<T>
where
    T: SelectType + Default,
{
    #[serde(serialize_with = "SelectType::serialize")]
    pub(super) r#type: PhantomData<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) option_groups: Vec<OptGroup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_user: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) response_url_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) filter: Option<ConversationFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_channel: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) placeholder: Option<PlainText>,
}

/// [Select menu element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element)
/// which can be one of the [Static options](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select),
/// [External data source](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select),
/// [User list](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select),
/// [Conversations list](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select),
/// or [Public channels](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SelectMenu {
    StaticOptions(Select<StaticOptions>),
    ExternalDataSource(Select<ExternalDataSource>),
    UserList(Select<UserList>),
    ConversationsList(Select<ConversationsList>),
    PublicChannels(Select<PublicChannels>),
}
