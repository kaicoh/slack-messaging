use serde::Serialize;

/// Type of conversation to set into [Conversation filter object](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Conversation {
    Im,
    Mpim,
    Private,
    Public,
}

/// [Conversation filter object](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object)
/// representation.
///
/// The Builder returns
/// [`ConversationFilterError`](crate::composition_objects::builders::ConversationFilterError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::{Conversation, ConversationFilter};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let filter = ConversationFilter::builder()
///     .include(Conversation::Public)
///     .include(Conversation::Mpim)
///     .exclude_bot_users(true)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "include": [
///         "public",
///         "mpim"
///     ],
///     "exclude_bot_users": true
/// });
///
/// let json = serde_json::to_value(filter).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ConversationFilter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) include: Vec<Conversation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) exclude_external_shared_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) exclude_bot_users: Option<bool>,
}
