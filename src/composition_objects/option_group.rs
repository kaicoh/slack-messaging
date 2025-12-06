use super::{Opt, PlainText, TextInOption};
use serde::Serialize;

/// [Option group object](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object)
/// representation.
///
/// The Builder returns [`OptGroupError`](crate::composition_objects::builders::OptGroupError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
/// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let options = OptGroup::<PlainText>::builder()
///     .label(plain_text!("Group One")?)
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-0")?)
///             .value("value-0")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-1")?)
///             .value("value-1")
///             .build()?
///     )
///     .build()?;
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
///
/// // If your object has any validation errors, the build method returns Result::Err
///
/// let options = OptGroup::<PlainText>::builder()
///     .label(plain_text!("Group One")?)
///     .build();
///
/// assert!(options.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct OptGroup<T: TextInOption> {
    pub(crate) label: Option<PlainText>,
    pub(crate) options: Vec<Opt<T>>,
}
