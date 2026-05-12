use crate::blocks::Block;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Legacy Slack attachment](https://api.slack.com/reference/messaging/attachments)
/// with support for colored sidebars and Block Kit content.
///
/// Attachments are the only way to render a colored sidebar in Slack messages.
/// They sit alongside (not inside) the `blocks` array at the message root level.
///
/// # Example
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::{Message, Attachment};
/// use slack_messaging::blocks::Section;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let message = Message::builder()
///     .attachment(
///         Attachment::builder()
///             .color("#e74c3c")
///             .block(
///                 Section::builder()
///                     .text(mrkdwn!("Critical finding detected")?)
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "attachments": [
///         {
///             "color": "#e74c3c",
///             "blocks": [
///                 {
///                     "type": "section",
///                     "text": {
///                         "type": "mrkdwn",
///                         "text": "Critical finding detected"
///                     }
///                 }
///             ]
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
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "block", validate("list::max_item_50"))]
    pub(crate) blocks: Option<Vec<Block>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) fallback: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::test_helpers::*;

    #[test]
    fn it_implements_builder() {
        let expected = Attachment {
            color: Some("#e74c3c".into()),
            blocks: Some(vec![section("hello world").into()]),
            fallback: Some("hello world".into()),
        };

        let val = Attachment::builder()
            .color("#e74c3c")
            .blocks(vec![section("hello world").into()])
            .fallback("hello world")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Attachment {
            color: Some("#2ecc71".into()),
            blocks: Some(vec![
                header("title").into(),
                section("content").into(),
            ]),
            fallback: None,
        };

        let val = Attachment::builder()
            .color("#2ecc71")
            .block(header("title"))
            .block(section("content"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_skips_none_fields() {
        let val = Attachment::builder().color("#f39c12").build().unwrap();
        let json = serde_json::to_value(val).unwrap();
        assert_eq!(
            json,
            serde_json::json!({ "color": "#f39c12" })
        );
    }
}
