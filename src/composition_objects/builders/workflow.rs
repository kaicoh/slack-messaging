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
#[derive(Debug, Clone, PartialEq, Default)]
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
    pub fn set_trigger(self, trigger: Option<Trigger>) -> Self {
        Self {
            trigger: new_trigger(trigger),
        }
    }

    /// set trigger filed value
    pub fn trigger(self, trigger: Trigger) -> Self {
        self.set_trigger(Some(trigger))
    }
}

fn new_trigger(trigger: Option<Trigger>) -> Value<Trigger> {
    pipe! { Value::new(trigger) => validators::required }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = Workflow {
            trigger: Some(trigger()),
        };

        let val = Workflow::builder()
            .set_trigger(Some(trigger()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Workflow::builder().trigger(trigger()).build().unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn trigger_field_is_required() {
        let err = Workflow::builder().build().unwrap_err();
        let expected = WorkflowError {
            trigger: vec![ValidationError::Required],
        };
        assert_eq!(err, expected);
    }
}
