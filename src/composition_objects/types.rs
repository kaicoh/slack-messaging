use crate::composition_objects::{MrkdwnText, PlainText, Text};
use crate::validators;

use derive_macro::Builder;
use serde::Serialize;
use serde_json::Value;

/// TextObject is a trait any text object representations must satisfy.
pub trait TextObject {
    fn text(&self) -> Option<&String>;
}

impl TextObject for Text {
    fn text(&self) -> Option<&String> {
        match self {
            Self::Plain(t) => t.text(),
            Self::Mrkdwn(t) => t.text(),
        }
    }
}

impl TextObject for MrkdwnText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

impl TextObject for PlainText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

/// Type of conversation to set into [Conversation filter object](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conversation {
    Im,
    Mpim,
    Private,
    Public,
}

/// Interaction type to set into [Dispatch action configuration](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    /// Represents `on_enter_pressed`.
    OnEnterPressed,

    /// Represents `on_character_entered`.
    OnCharacterEntered,
}

/// TextInOption is a trait that can be set to text and desciption field
/// of [`Opt`](crate::composition_objects::Opt) object
pub trait TextInOption: TextObject {}

impl TextInOption for Text {}
impl TextInOption for PlainText {}

/// Phantom type to control url field of [`Opt`](crate::composition_objects::Opt). By default, this type is used,
/// and the url field is unavailable.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlUnavailable;

/// Phantom type to control url field of [`Opt`](crate::composition_objects::Opt). Using this type, the url field
/// is available.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlAvailable;

/// Input parameter for [Trigger object](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object).
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::types::InputParameter;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let param = InputParameter::builder()
///     .name("input_parameter_a")
///     .value("Value for input param A")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "name": "input_parameter_a",
///     "value": "Value for input param A"
/// });
///
/// let json = serde_json::to_value(param).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let param = InputParameter::builder()
///     .name("input_parameter_a")
///     .build();
///
/// assert!(param.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct InputParameter {
    #[builder(setter = "set_name")]
    pub(crate) name: Option<String>,

    #[builder(setter = "set_value")]
    pub(crate) value: Option<Value>,
}

fn set_name(value: Option<String>) -> crate::value::Value<String> {
    pipe! { crate::value::Value::new(value) => validators::required }
}

fn set_value(value: Option<Value>) -> crate::value::Value<Value> {
    pipe! { crate::value::Value::new(value) => validators::required }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = InputParameter {
            name: Some("input_parameter_a".into()),
            value: Some(Value::String("Value for input param A".into())),
        };

        let val = InputParameter::builder()
            .set_name(Some("input_parameter_a"))
            .set_value(Some(Value::String("Value for input param A".into())))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = InputParameter::builder()
            .name("input_parameter_a")
            .value("Value for input param A")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_name_field() {
        let err = InputParameter::builder()
            .value("Value for input param A")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "InputParameter");

        let errors = err.field("name");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_value_field() {
        let err = InputParameter::builder()
            .name("input_parameter_a")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "InputParameter");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
