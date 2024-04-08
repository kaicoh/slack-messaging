use super::Trigger;
use serde::Serialize;

/// [Workflow object](https://api.slack.com/reference/block-kit/composition-objects#workflow)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
/// let workflow = Workflow::builder()
///     .trigger(
///         Trigger::builder()
///              .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///              .customizable_input_parameter(
///                  InputParameter::builder()
///                      .name("input_parameter_a")
///                      .value("Value for input param A")
///                      .build()
///              )
///              .customizable_input_parameter(
///                  InputParameter::builder()
///                      .name("input_parameter_b")
///                      .value("Value for input param B")
///                      .build()
///              )
///              .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "trigger": {
///         "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
///         "customizable_input_parameters": [
///             {
///                 "name": "input_parameter_a",
///                 "value": "Value for input param A"
///             },
///             {
///                 "name": "input_parameter_b",
///                 "value": "Value for input param B"
///             }
///         ]
///     }
/// });
///
/// let json = serde_json::to_value(workflow).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Workflow {
    pub(super) trigger: Trigger,
}
