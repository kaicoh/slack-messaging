use crate::composition_objects::{PlainText, Workflow};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Workflow button element](https://docs.slack.dev/reference/block-kit/block-elements/workflow-button-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::WorkflowButton;
/// use slack_messaging::composition_objects::{types::InputParameter, Trigger, Workflow};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = WorkflowButton::builder()
///     .text(plain_text!("Run Workflow")?)
///     .action_id("workflowbutton123")
///     .workflow(
///         Workflow::builder()
///             .trigger(
///                 Trigger::builder()
///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
///                      .customizable_input_parameter(
///                          InputParameter::builder()
///                              .name("input_parameter_a")
///                              .value("Value for input param A")
///                              .build()?
///                      )
///                      .customizable_input_parameter(
///                          InputParameter::builder()
///                              .name("input_parameter_b")
///                              .value("Value for input param B")
///                              .build()?
///                      )
///                      .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "workflow_button",
///     "text": {
///         "type": "plain_text",
///         "text": "Run Workflow"
///     },
///     "action_id": "workflowbutton123",
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
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = WorkflowButton::builder()
///     .text(plain_text!("Run Workflow")?)
///     .action_id("workflowbutton123")
///     .build();
///
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "workflow_button")]
pub struct WorkflowButton {
    #[builder(validate("required", "text_object::max_75"))]
    pub(crate) text: Option<PlainText>,

    #[builder(validate("required", "text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[builder(validate("required"))]
    pub(crate) workflow: Option<Workflow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(private_setter)]
    pub(crate) style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_75"))]
    pub(crate) accessibility_label: Option<String>,
}

impl WorkflowButtonBuilder {
    /// set "primary" to style field
    pub fn primary(self) -> Self {
        self.style("primary")
    }

    /// set "danger" to style field
    pub fn danger(self) -> Self {
        self.style("danger")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = WorkflowButton {
            text: Some(plain_text("Run Workflow")),
            action_id: Some("workflow_button_0".into()),
            workflow: Some(workflow()),
            style: Some("primary"),
            accessibility_label: Some("Run Workflow!".into()),
        };

        let val = WorkflowButton::builder()
            .set_text(Some(plain_text("Run Workflow")))
            .set_action_id(Some("workflow_button_0"))
            .set_workflow(Some(workflow()))
            .primary()
            .set_accessibility_label(Some("Run Workflow!"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let expected = WorkflowButton {
            style: Some("danger"),
            ..expected
        };

        let val = WorkflowButton::builder()
            .text(plain_text("Run Workflow"))
            .action_id("workflow_button_0")
            .workflow(workflow())
            .danger()
            .accessibility_label("Run Workflow!")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requries_text_field() {
        let err = WorkflowButton::builder()
            .action_id("workflow_button_0")
            .workflow(workflow())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_length_less_than_75_characters_long() {
        let err = WorkflowButton::builder()
            .text(plain_text("a".repeat(76)))
            .action_id("workflow_button_0")
            .workflow(workflow())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
    }

    #[test]
    fn it_requries_action_id_field() {
        let err = WorkflowButton::builder()
            .text(plain_text("Run Workflow"))
            .workflow(workflow())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requries_action_id_less_than_255_characters_long() {
        let err = WorkflowButton::builder()
            .text(plain_text("Run Workflow"))
            .action_id("a".repeat(256))
            .workflow(workflow())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requries_workflow_field() {
        let err = WorkflowButton::builder()
            .text(plain_text("Run Workflow"))
            .action_id("workflow_button_0")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("workflow");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requries_accessibility_label_less_than_75_characters_long() {
        let err = WorkflowButton::builder()
            .text(plain_text("Run Workflow"))
            .action_id("workflow_button_0")
            .workflow(workflow())
            .accessibility_label("a".repeat(76))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "WorkflowButton");

        let errors = err.field("accessibility_label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
    }
}
