use crate::blocks::Block;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [`Message`](https://docs.slack.dev/messaging#payloads)
/// representation.
///
/// # Example
///
/// See also [Header](crate::blocks::Header), [Section](crate::blocks::Section)
/// and [any other blocks](crate::blocks) to know how to build these blocks.
///
/// Every block and its components have each builder and their build method
/// returns [Result].
///
/// For example, according to [the official document](https://docs.slack.dev/reference/block-kit/blocks?available-in-surfaces=Home+tabs),
/// you can include up to 50 blocks in each message. If you include more than 50
/// blocks in a message, the build method of [MessageBuilder] returns Result::Err.
///
/// ```
/// use slack_messaging::{mrkdwn, plain_text, Message};
/// use slack_messaging::blocks::{Header, Section};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let message = Message::builder()
///     .text("New Paid Time Off request from Fred Enriquez")
///     .block(
///         Header::builder()
///             .text(plain_text!("New request")?)
///             .build()?
///     )
///     .block(
///         Section::builder()
///             .field(mrkdwn!("*Type:*\nPaid Time Off")?)
///             .field(mrkdwn!("*Created by:*\n<example.com|Fred Enriquez>")?)
///             .build()?
///     )
///     .block(
///         Section::builder()
///             .field(mrkdwn!("*When:*\nAug 10 - Aug 13")?)
///             .build()?
///     )
///     .block(
///         Section::builder()
///             .text(mrkdwn!("<https://example.com|View request>")?)
///             .build()?
///     )
///     .build()?;
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
/// let json = serde_json::to_value(message)?;
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "block", validate("list::max_item_50"))]
    pub(crate) blocks: Option<Vec<Block>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) thread_ts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mrkdwn: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) response_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) replace_original: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) delete_original: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) reply_broadcast: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Message {
            text: Some("some text".into()),
            blocks: Some(vec![
                header("this is a header block").into(),
                section("this is a section block").into(),
            ]),
            thread_ts: Some("thread ts".into()),
            mrkdwn: Some(true),
            response_type: Some("response type".into()),
            replace_original: Some(true),
            delete_original: Some(true),
            reply_broadcast: Some(true),
        };

        let val = Message::builder()
            .set_text(Some("some text"))
            .set_blocks(Some(vec![
                header("this is a header block").into(),
                section("this is a section block").into(),
            ]))
            .set_thread_ts(Some("thread ts"))
            .set_mrkdwn(Some(true))
            .set_response_type(Some("response type"))
            .set_replace_original(Some(true))
            .set_delete_original(Some(true))
            .set_reply_broadcast(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Message::builder()
            .text("some text")
            .blocks(vec![
                header("this is a header block").into(),
                section("this is a section block").into(),
            ])
            .thread_ts("thread ts")
            .mrkdwn(true)
            .response_type("response type")
            .replace_original(true)
            .delete_original(true)
            .reply_broadcast(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_impelements_push_item_method() {
        let expected = Message {
            text: None,
            blocks: Some(vec![
                header("this is a header block").into(),
                section("this is a section block").into(),
            ]),
            thread_ts: None,
            mrkdwn: None,
            response_type: None,
            replace_original: None,
            delete_original: None,
            reply_broadcast: None,
        };

        let val = Message::builder()
            .block(header("this is a header block"))
            .block(section("this is a section block"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requries_blocks_list_size_less_than_50() {
        let blocks: Vec<Block> = (0..51).map(|_| section("some section").into()).collect();
        let err = Message::builder().blocks(blocks).build().unwrap_err();
        assert_eq!(err.object(), "Message");

        let errors = err.field("blocks");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(50)));
    }
}
