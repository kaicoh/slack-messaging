use super::error::ValidationError;
use super::validators;
use super::value::Value;
use super::{Builder, Trigger, Workflow};

use std::error::Error;
use std::fmt;

impl Workflow {
    /// Construct a [`WorkflowBuilder`].
    pub fn builder() -> WorkflowBuilder {
        WorkflowBuilder::default()
    }
}

/// Error while building [`Workflow`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowError {
    /// errors of trigger field
    pub trigger: Vec<ValidationError>,
}

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WorkflowError {{ trigger: {:?} }}", self.trigger)
    }
}

impl Error for WorkflowError {}

/// Builder for [`Workflow`] object.
#[derive(Debug)]
pub struct WorkflowBuilder {
    trigger: Value<Trigger>,
}

impl Default for WorkflowBuilder {
    fn default() -> Self {
        WorkflowBuilder {
            trigger: new_trigger(None),
        }
    }
}

impl Builder for WorkflowBuilder {
    type Target = Workflow;
    type Error = WorkflowError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        if self.trigger.has_errors() {
            Err(WorkflowError {
                trigger: self.trigger.errors,
            })
        } else {
            Ok(Workflow {
                trigger: self.trigger.inner,
            })
        }
    }
}

impl WorkflowBuilder {
    /// get trigger filed value
    pub fn get_trigger(&self) -> Option<&Trigger> {
        self.trigger.inner_ref()
    }

    /// set trigger filed value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger, Workflow};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let workflow = Workflow::builder()
    ///     .set_trigger(
    ///         Some(
    ///             Trigger::builder()
    ///                  .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///                  .customizable_input_parameter(
    ///                      InputParameter::builder()
    ///                          .name("input_parameter_a")
    ///                          .value("Value for input param A")
    ///                          .build()?
    ///                  )
    ///                  .customizable_input_parameter(
    ///                      InputParameter::builder()
    ///                          .name("input_parameter_b")
    ///                          .value("Value for input param B")
    ///                          .build()?
    ///                  )
    ///                  .build()?
    ///         )
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
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_trigger(self, trigger: Option<Trigger>) -> Self {
        Self {
            trigger: new_trigger(trigger),
        }
    }

    /// set trigger filed value
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
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn trigger(self, trigger: Trigger) -> Self {
        self.set_trigger(Some(trigger))
    }
}

fn new_trigger(trigger: Option<Trigger>) -> Value<Trigger> {
    pipe! { Value::new(trigger) => validators::required }
}

#[cfg(test)]
mod tests {
    use super::super::InputParameter;
    use super::*;

    #[test]
    fn it_builds_workflow() {
        let result = Workflow::builder()
            .trigger(
                Trigger::builder()
                    .url("foo")
                    .customizable_input_parameter(
                        InputParameter::builder()
                            .name("bar")
                            .value("baz")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = Workflow {
            trigger: Some(Trigger {
                url: Some("foo".into()),
                customizable_input_parameters: vec![InputParameter {
                    name: Some("bar".into()),
                    value: Some("baz".into()),
                }],
            }),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_buildere_returns_error() {
        let result = Workflow::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = WorkflowError {
            trigger: vec![ValidationError::Required],
        };
        assert_eq!(err, expected);
    }
}
