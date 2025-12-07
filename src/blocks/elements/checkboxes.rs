use super::composition_objects::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

/// [Checkboxes](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
/// representation.
///
/// The Builder returns [`CheckboxesError`](crate::blocks::elements::builders::CheckboxesError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, mrkdwn};
/// use slack_messaging::blocks::elements::Checkboxes;
/// use slack_messaging::composition_objects::{Opt, Text};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let checkboxes = Checkboxes::builder()
///     .action_id("group-0")
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("option-0")?)
///             .value("value-0")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("option-1")?)
///             .value("value-1")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "checkboxes",
///     "action_id": "group-0",
///     "options": [
///         {
///             "value": "value-0",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "option-0"
///             }
///         },
///         {
///             "value": "value-1",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "option-1"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(checkboxes).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let checkboxes = Checkboxes::builder()
///     .action_id("group-0")
///     .build();
///
/// assert!(checkboxes.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "checkboxes")]
pub struct Checkboxes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    pub(crate) options: Vec<Opt<Text>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) initial_options: Vec<Opt<Text>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,
}
