use crate::composition_objects::TextContent;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Alert block](https://docs.slack.dev/reference/block-kit/blocks/alert-block) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/alert-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | [TextContent] | Yes | N/A |
/// | level | [AlertLevel] | No | N/A |
/// | block_id | String | No | Maximum 255 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::{Alert, AlertLevel};
/// use slack_messaging::mrkdwn;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let alert = Alert::builder()
///     .text(mrkdwn!("The work is mysterious and important.")?)
///     .level(AlertLevel::Info)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "alert",
///     "text": {
///         "type": "mrkdwn",
///         "text": "The work is mysterious and important."
///     },
///     "level": "info"
/// });
///
/// let json = serde_json::to_value(alert).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "alert")]
pub struct Alert {
    #[builder(validate("required"))]
    pub(crate) text: Option<TextContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) level: Option<AlertLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

/// Values that can be set to the `level` field of [Alert].
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AlertLevel {
    Default,
    Info,
    Warning,
    Error,
    Success,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Alert {
            text: Some(plain_text("The work is mysterious and important.").into()),
            level: Some(AlertLevel::Info),
            block_id: Some("alert_block_0".into()),
        };

        let val = Alert::builder()
            .set_text(Some(plain_text("The work is mysterious and important.")))
            .set_level(Some(AlertLevel::Info))
            .set_block_id(Some("alert_block_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Alert::builder()
            .text(plain_text("The work is mysterious and important."))
            .level(AlertLevel::Info)
            .block_id("alert_block_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = Alert::builder().build().unwrap_err();
        assert_eq!(err.object(), "Alert");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Alert::builder()
            .text(plain_text("The work is mysterious and important."))
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Alert");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
