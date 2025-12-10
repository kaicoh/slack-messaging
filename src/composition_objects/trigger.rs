use crate::composition_objects::types::InputParameter;
use crate::validators;
use crate::value::Value;

use derive_macro::Builder;
use serde::Serialize;

/// [Trigger object](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::{types::InputParameter, Trigger};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let trigger = Trigger::builder()
///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///     .customizable_input_parameter(
///         InputParameter::builder()
///             .name("input_parameter_a")
///             .value("Value for input param A")
///             .build()?
///     )
///     .customizable_input_parameter(
///         InputParameter::builder()
///             .name("input_parameter_b")
///             .value("Value for input param B")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
///     "customizable_input_parameters": [
///         {
///             "name": "input_parameter_a",
///             "value": "Value for input param A"
///         },
///         {
///             "name": "input_parameter_b",
///             "value": "Value for input param B"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(trigger).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let trigger = Trigger::builder()
///     .customizable_input_parameter(
///         InputParameter::builder()
///             .name("input_parameter_a")
///             .value("Value for input param A")
///             .build()?
///     )
///     .build();
///
/// assert!(trigger.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct Trigger {
    #[builder(setter = "set_url")]
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "customizable_input_parameter")]
    pub(crate) customizable_input_parameters: Option<Vec<InputParameter>>,
}

fn set_url(value: Option<String>) -> Value<String> {
    pipe! { Value::new(value) => validators::required }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Trigger {
            url: Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz".into()),
            customizable_input_parameters: Some(vec![
                input_param("param_0", "value_0"),
                input_param("param_1", "value_1"),
            ]),
        };

        let val = Trigger::builder()
            .set_url(Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz"))
            .set_customizable_input_parameters(Some(vec![
                input_param("param_0", "value_0"),
                input_param("param_1", "value_1"),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Trigger::builder()
            .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
            .customizable_input_parameters(vec![
                input_param("param_0", "value_0"),
                input_param("param_1", "value_1"),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Trigger {
            url: Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz".into()),
            customizable_input_parameters: Some(vec![
                input_param("param_0", "value_0"),
                input_param("param_1", "value_1"),
            ]),
        };

        let val = Trigger::builder()
            .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
            .customizable_input_parameter(input_param("param_0", "value_0"))
            .customizable_input_parameter(input_param("param_1", "value_1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_url_field() {
        let err = Trigger::builder()
            .customizable_input_parameter(input_param("param_0", "value_0"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Trigger");

        let errors = err.field("url");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
