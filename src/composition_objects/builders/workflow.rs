use super::{Trigger, Workflow};

impl Workflow {
    /// Construct a [`WorkflowBuilder`].
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }
}

/// Builder for [`Workflow`] object.
#[derive(Debug, Default)]
pub struct WorkflowBuilder {
    trigger: Option<Trigger>,
}

impl WorkflowBuilder {
    /// Set trigger field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
    /// let workflow = Workflow::builder()
    ///     .set_trigger(
    ///         Some(Trigger::builder()
    ///              .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///              .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "trigger": {
    ///         "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(workflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_trigger(self, trigger: Option<Trigger>) -> Self {
        Self { trigger }
    }

    /// Set trigger field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
    /// let workflow = Workflow::builder()
    ///     .trigger(
    ///         Trigger::builder()
    ///              .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///              .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "trigger": {
    ///         "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(workflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn trigger(self, trigger: Trigger) -> Self {
        self.set_trigger(Some(trigger))
    }

    /// Build a [`Workflow`] object. This method will panic if `trigger` is not set.
    pub fn build(self) -> Workflow {
        Workflow {
            trigger: self
                .trigger
                .expect("trigger must be set to WorkflowBuilder"),
        }
    }

    /// Get trigger value.
    pub fn get_trigger(&self) -> &Option<Trigger> {
        &self.trigger
    }
}
