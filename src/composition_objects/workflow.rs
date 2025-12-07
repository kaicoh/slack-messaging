use super::Trigger;
use serde::Serialize;

/// [Workflow object](https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object)
/// representation.
///
/// The Builder returns [`WorkflowError`](crate::composition_objects::builders::WorkflowError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let workflow = Workflow::builder()
///     .trigger(
///         Trigger::builder()
///              .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///              .customizable_input_parameter(
///                  InputParameter::builder()
///                      .name("input_parameter_a")
///                      .value("Value for input param A")
///                      .build()?
///              )
///              .customizable_input_parameter(
///                  InputParameter::builder()
///                      .name("input_parameter_b")
///                      .value("Value for input param B")
///                      .build()?
///              )
///              .build()?
///     )
///     .build()?;
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
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let workflow = Workflow::builder().build();
/// assert!(workflow.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Workflow {
    pub(crate) trigger: Option<Trigger>,
}
