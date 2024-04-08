use super::InputParameter;
use serde::Serialize;

/// [Trigger object](https://api.slack.com/reference/block-kit/composition-objects#trigger)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::{InputParameter, Trigger};
/// let trigger = Trigger::builder()
///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///     .customizable_input_parameter(
///         InputParameter::builder()
///             .name("input_parameter_a")
///             .value("Value for input param A")
///             .build()
///     )
///     .customizable_input_parameter(
///         InputParameter::builder()
///             .name("input_parameter_b")
///             .value("Value for input param B")
///             .build()
///     )
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Trigger {
    pub(super) url: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) customizable_input_parameters: Vec<InputParameter>,
}
