use crate::composition_objects::Trigger;
use crate::validators::required;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Workflow object](https://docs.slack.dev/reference/block-kit/composition-objects/workflow-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::{types::InputParameter, Trigger, Workflow};
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
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct Workflow {
    #[builder(validate("required"))]
    pub(crate) trigger: Option<Trigger>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
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
    fn it_requires_trigger_field() {
        let err = Workflow::builder().build().unwrap_err();
        assert_eq!(err.object(), "Workflow");

        let errors = err.field("trigger");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
