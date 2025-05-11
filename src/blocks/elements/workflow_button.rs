use super::composition_objects::{Text, Workflow};
use serde::Serialize;

/// [Workflow button element](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::WorkflowButton;
/// # use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
/// let button = WorkflowButton::builder()
///     .text("Run Workflow")
///     .workflow(
///         Workflow::builder()
///             .trigger(
///                 Trigger::builder()
///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///                      .customizable_input_parameter(
///                          InputParameter::builder()
///                              .name("input_parameter_a")
///                              .value("Value for input param A")
///                              .build()
///                      )
///                      .customizable_input_parameter(
///                          InputParameter::builder()
///                              .name("input_parameter_b")
///                              .value("Value for input param B")
///                              .build()
///                      )
///                      .build()
///             )
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "workflow_button",
///     "text": {
///         "type": "plain_text",
///         "text": "Run Workflow"
///     },
///     "workflow": {
///         "trigger": {
///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
///             "customizable_input_parameters": [
///                 {
///                     "name": "input_parameter_a",
///                     "value": "Value for input param A"
///                 },
///                 {
///                     "name": "input_parameter_b",
///                     "value": "Value for input param B"
///                 }
///             ]
///         }
///     }
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct WorkflowButton {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) text: Text,

    pub(super) workflow: Workflow,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) accessibility_label: Option<String>,
}
