use super::composition_objects::{
    ConfirmationDialog, ConversationFilter, Opt, OptGroup, PlainText,
};
use super::select_menu_types::{
    Conversations, ExternalDataSource, PublicChannels, SelectType, StaticOptions, Users,
};
use serde::Serialize;
use std::marker::PhantomData;

/// The entry point to build any variant of [`SelectMenu`].
///
/// # Example
///
/// ## [Static options](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select)
///
/// ```
/// # use slack_messaging::blocks::elements::Select;
/// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
/// # use slack_messaging::composition_objects::Opt;
/// # use slack_messaging::plain_text;
/// let menu = Select::<StaticOptions>::builder()
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
///
/// ## [External data source](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
///
/// ```
/// # use slack_messaging::blocks::elements::Select;
/// # use slack_messaging::blocks::elements::select_menu_types::ExternalDataSource;
/// let menu = Select::<ExternalDataSource>::builder()
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
///
/// ## [User list](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select)
///
/// ```
/// # use slack_messaging::blocks::elements::Select;
/// # use slack_messaging::blocks::elements::select_menu_types::Users;
/// let menu = Select::<Users>::builder()
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
///
/// ## [Conversation list](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
///
/// ```
/// # use slack_messaging::blocks::elements::Select;
/// # use slack_messaging::blocks::elements::select_menu_types::Conversations;
/// let menu = Select::<Conversations>::builder()
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
///
/// ## [Public channels](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
///
/// ```
/// # use slack_messaging::blocks::elements::Select;
/// # use slack_messaging::blocks::elements::select_menu_types::PublicChannels;
/// let menu = Select::<PublicChannels>::builder()
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

/// [Select menu element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element).
/// To build any of this variant, you should use [`Select`].
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SelectMenu {
    /// [Select menu of static options element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#static_select)
    /// representation.
    StaticOptions(Select<StaticOptions>),

    /// [Select menu of external data source element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
    /// representation.
    ExternalDataSource(Select<ExternalDataSource>),

    /// [Select menu of users element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#users_select)
    /// representation.
    Users(Select<Users>),

    /// [Select menu of conversations element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
    /// representation.
    Conversations(Select<Conversations>),

    /// [Select menu of public channels element](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#channels_select)
    /// representation.
    PublicChannels(Select<PublicChannels>),
}
