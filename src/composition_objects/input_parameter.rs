use serde::Serialize;
use serde_json::Value;

/// [Input parameter object](https://api.slack.com/reference/block-kit/composition-objects#input_parameter) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::InputParameter;
/// let param = InputParameter::builder()
///     .name("input_parameter_a")
///     .value("Value for input param A")
///     .build();
///
/// let expected = serde_json::json!({
///     "name": "input_parameter_a",
///     "value": "Value for input param A"
/// });
///
/// let json = serde_json::to_value(param).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct InputParameter {
    pub(super) name: String,
    pub(super) value: Value,
}
