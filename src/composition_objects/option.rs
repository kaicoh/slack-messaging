use super::PlainText;
use serde::Serialize;

/// [Option object](https://docs.slack.dev/reference/block-kit/composition-objects/option-object)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::{Opt, PlainText};
/// # use slack_messaging::plain_text;
/// let option = Opt::<PlainText>::builder()
///     .text(plain_text!("Maru"))
///     .value("maru")
///     .build();
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Maru"
///     },
///     "value": "maru"
/// });
///
/// let json = serde_json::to_value(option).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Opt<T = PlainText>
where
    T: Clone + Serialize,
{
    pub(super) text: T,

    pub(super) value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) description: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) url: Option<String>,
}
