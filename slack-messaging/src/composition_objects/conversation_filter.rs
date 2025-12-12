use crate::composition_objects::types::Conversation;
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Conversation filter object](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::{types::Conversation, ConversationFilter};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let filter = ConversationFilter::builder()
///     .conversation(Conversation::Public)
///     .conversation(Conversation::Mpim)
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
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let filter = ConversationFilter::builder().build();
/// assert!(filter.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[builder(validate = "validate")]
pub struct ConversationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "conversation", validate("list::not_empty"))]
    pub(crate) include: Option<Vec<Conversation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_external_shared_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) exclude_bot_users: Option<bool>,
}

fn validate(builder: &ConversationFilterBuilder) -> Vec<ValidationErrorKind> {
    if builder.get_include().is_none()
        && builder.get_exclude_external_shared_channels().is_none()
        && builder.get_exclude_bot_users().is_none()
    {
        vec![ValidationErrorKind::NoFieldProvided]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = ConversationFilter {
            include: Some(vec![Conversation::Im, Conversation::Private]),
            exclude_external_shared_channels: Some(false),
            exclude_bot_users: Some(true),
        };

        let filter = ConversationFilter::builder()
            .set_include(Some(vec![Conversation::Im, Conversation::Private]))
            .set_exclude_external_shared_channels(Some(false))
            .set_exclude_bot_users(Some(true))
            .build()
            .unwrap();

        assert_eq!(filter, expected);

        let filter = ConversationFilter::builder()
            .include(vec![Conversation::Im, Conversation::Private])
            .exclude_external_shared_channels(false)
            .exclude_bot_users(true)
            .build()
            .unwrap();

        assert_eq!(filter, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = ConversationFilter {
            include: Some(vec![Conversation::Im, Conversation::Private]),
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
        };

        let filter = ConversationFilter::builder()
            .conversation(Conversation::Im)
            .conversation(Conversation::Private)
            .build()
            .unwrap();

        assert_eq!(filter, expected);
    }

    #[test]
    fn it_requires_at_least_one_field() {
        let err = ConversationFilter::builder().build().unwrap_err();
        assert_eq!(err.object(), "ConversationFilter");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::NoFieldProvided));

        let filter = ConversationFilter::builder()
            .conversation(Conversation::Public)
            .build();
        assert!(filter.is_ok());

        let filter = ConversationFilter::builder()
            .exclude_external_shared_channels(true)
            .build();
        assert!(filter.is_ok());

        let filter = ConversationFilter::builder()
            .exclude_bot_users(true)
            .build();
        assert!(filter.is_ok());
    }

    #[test]
    fn it_requires_include_field_not_empty() {
        let err = ConversationFilter::builder()
            .include(vec![])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConversationFilter");

        let errors = err.field("include");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }
}
