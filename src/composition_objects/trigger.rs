use serde::Serialize;
use serde_json::Value;

/// Input parameter for [Trigger object](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object).
///
/// The Builder returns [`InputParameterError`](crate::composition_objects::builders::InputParameterError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::InputParameter;
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
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct InputParameter {
    pub(crate) name: Option<String>,
    pub(crate) value: Option<Value>,
}

/// [Trigger object](https://docs.slack.dev/reference/block-kit/composition-objects/trigger-object)
/// representation.
///
/// The Builder returns [`TriggerError`](crate::composition_objects::builders::TriggerError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::{InputParameter, Trigger};
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
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Trigger {
    pub(crate) url: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) customizable_input_parameters: Vec<InputParameter>,
}
