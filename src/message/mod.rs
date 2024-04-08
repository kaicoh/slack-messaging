/// Builder object for Message.
pub mod builder;

use super::blocks::Block;
use serde::Serialize;

/// [Message payload](https://api.slack.com/reference/messaging/payload)
/// representation.
///
/// # Example
///
/// See also [Header](crate::blocks::Header), [Section](crate::blocks::Section)
/// and [any other blocks](crate::blocks) to know how to build these blocks.
///
/// ```
/// use slack_messaging::{blocks::{Header, Section}, mrkdwn, Message};
///
/// let message = Message::builder()
///     .text("New Paid Time Off request from Fred Enriquez")
///     .block(
///         Header::builder()
///             .text("New request")
///             .build()
///     )
///     .block(
///         Section::builder()
///             .field(mrkdwn!("*Type:*\nPaid Time Off"))
///             .field(mrkdwn!("*Created by:*\n<example.com|Fred Enriquez>"))
///             .build()
///     )
///     .block(
///         Section::builder()
///             .field(mrkdwn!("*When:*\nAug 10 - Aug 13"))
///             .build()
///     )
///     .block(
///         Section::builder()
///             .text(mrkdwn!("<https://example.com|View request>"))
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "text": "New Paid Time Off request from Fred Enriquez",
///     "blocks": [
///         {
///             "type": "header",
///             "text": {
///                 "type": "plain_text",
///                 "text": "New request"
///             }
///         },
///         {
///             "type": "section",
///             "fields": [
///                 {
///                     "type": "mrkdwn",
///                     "text": "*Type:*\nPaid Time Off"
///                 },
///                 {
///                     "type": "mrkdwn",
///                     "text": "*Created by:*\n<example.com|Fred Enriquez>"
///                 }
///             ]
///         },
///         {
///             "type": "section",
///             "fields": [
///                 {
///                     "type": "mrkdwn",
///                     "text": "*When:*\nAug 10 - Aug 13"
///                 }
///             ]
///         },
///         {
///             "type": "section",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "<https://example.com|View request>"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(message).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) text: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) blocks: Vec<Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) thread_ts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) mrkdwn: Option<bool>,
}
