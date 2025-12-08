use super::types::{TextInOption, UrlUnavailable};
use serde::Serialize;
use std::marker::PhantomData;

/// [Option object](https://docs.slack.dev/reference/block-kit/composition-objects/option-object)
/// representation.
///
/// The Builder returns [`OptError`](crate::composition_objects::builders::OptError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
/// use slack_messaging::composition_objects::{Opt, PlainText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let option = Opt::<PlainText>::builder()
///     .text(plain_text!("Maru")?)
///     .value("maru")
///     .build()?;
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
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let option = Opt::<PlainText>::builder()
///     .text(plain_text!("Maru")?)
///     .build();
///
/// assert!(option.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Opt<T: TextInOption, P = UrlUnavailable> {
    #[serde(skip)]
    pub(crate) phantom: PhantomData<P>,

    pub(crate) text: Option<T>,

    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) description: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
}
