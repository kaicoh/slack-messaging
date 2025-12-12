use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [broadcast element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#broadcast-element-type)
/// for rich text.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextElementBroadcast, BroadcastRange};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let element = RichTextElementBroadcast::builder()
///     .range(BroadcastRange::Everyone)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "broadcast",
///     "range": "everyone"
/// });
///
/// let json = serde_json::to_value(element).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let element = RichTextElementBroadcast::builder().build();
/// assert!(element.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "broadcast")]
pub struct RichTextElementBroadcast {
    #[builder(validate("required"))]
    pub(crate) range: Option<BroadcastRange>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextElementBroadcast {
            range: Some(BroadcastRange::Channel),
        };

        let val = RichTextElementBroadcast::builder()
            .set_range(Some(BroadcastRange::Channel))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextElementBroadcast::builder()
            .range(BroadcastRange::Channel)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_range_field() {
        let err = RichTextElementBroadcast::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextElementBroadcast");

        let errors = err.field("range");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}

/// The range of broadcast for
/// [`RichTextElementBroadcast`](crate::blocks::rich_text::element_types::RichTextElementBroadcast) element.
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BroadcastRange {
    /// notifies only the active members of a channel.
    Here,
    /// notifies all members of a channel.
    Channel,
    /// notifies every person in the #general channel.
    Everyone,
}
