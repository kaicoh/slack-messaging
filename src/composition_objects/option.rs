use super::{PlainText, Text, TextObject};
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

/// TextInOption is a trait that can be set to text and desciption field
/// of [`Opt`] object
pub trait TextInOption: TextObject {}

impl TextInOption for Text {}
impl TextInOption for PlainText {}

/// Phantom type to control url field of [`Opt`]. By default, this type is used,
/// and the url field is unavailable.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlUnavailable;

/// Phantom type to control url field of [`Opt`]. Using this type, the url field
/// is available.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlAvailable;
