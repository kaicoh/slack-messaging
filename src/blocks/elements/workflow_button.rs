use super::{Text, Workflow};
use serde::Serialize;

/// [Workflow button element](https://api.slack.com/reference/block-kit/block-elements#workflow_button)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{InputParameter, Trigger, Workflow, WorkflowButton};
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
    kind: &'static str,

    text: Text,

    workflow: Workflow,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_label: Option<String>,
}

impl WorkflowButton {
    /// Construct a [`WorkflowButtonBuilder`].
    pub fn builder() -> WorkflowButtonBuilder {
        WorkflowButtonBuilder::default()
    }
}

/// Builder for [`Button`] object.
#[derive(Debug, Default)]
pub struct WorkflowButtonBuilder {
    text: Option<Text>,
    workflow: Option<Workflow>,
    action_id: Option<String>,
    style: Option<&'static str>,
    accessibility_label: Option<String>,
}

impl WorkflowButtonBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(text.into()).build();
        self.set_text(Some(text))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .set_text(Some(plain_text!("Click Me")))
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<Text>) -> Self {
        Self { text, ..self }
    }

    /// Set workflow field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn workflow(self, workflow: Workflow) -> Self {
        self.set_workflow(Some(workflow))
    }

    /// Set workflow field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .set_workflow(
    ///         Some(Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build())
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_workflow(self, workflow: Option<Workflow>) -> Self {
        Self { workflow, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .action_id("click_me_123")
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "click_me_123",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .set_action_id(Some("click_me_123".into()))
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "click_me_123",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set primary to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .primary()
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "primary",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    /// Set danger to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .danger()
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "danger",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .accessibility_label("Click Me")
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label.into()))
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Trigger, Workflow, WorkflowButton};
    /// let button = WorkflowButton::builder()
    ///     .text("Click Me")
    ///     .set_accessibility_label(Some("Click Me".into()))
    ///     .workflow(
    ///         Workflow::builder()
    ///             .trigger(
    ///                 Trigger::builder()
    ///                      .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                      .build()
    ///             )
    ///             .build()
    ///
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "workflow_button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me",
    ///     "workflow": {
    ///         "trigger": {
    ///             "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_accessibility_label(self, label: Option<String>) -> Self {
        Self {
            accessibility_label: label,
            ..self
        }
    }

    /// Build a [`WorkflowButton`] object. This method will panic if `text` and `workflow` are not set.
    pub fn build(self) -> WorkflowButton {
        WorkflowButton {
            kind: "workflow_button",
            text: self
                .text
                .expect("text must be set to WorkflowButtonBuilder"),
            workflow: self
                .workflow
                .expect("workflow must be set to WorkflowButtonBuilder"),
            action_id: self.action_id,
            style: self.style,
            accessibility_label: self.accessibility_label,
        }
    }
}
