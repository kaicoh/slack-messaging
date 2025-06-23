use super::{Opt, PlainText};
use serde::Serialize;

/// [Option group object](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::{OptGroup, Opt};
/// # use slack_messaging::plain_text;
/// let options = OptGroup::builder()
///     .label("Group One")
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
///     .build();
///
/// let expected = serde_json::json!({
///     "label": {
///         "type": "plain_text",
///         "text": "Group One"
///     },
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0",
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         },
///     ]
/// });
///
/// let json = serde_json::to_value(options).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct OptGroup {
    pub(super) label: PlainText,
    pub(super) options: Vec<Opt>,
}
