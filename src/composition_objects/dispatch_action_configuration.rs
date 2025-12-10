use crate::composition_objects::types::TriggerAction;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Dispatch action configuration](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::{DispatchActionConfiguration, types::TriggerAction};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let config = DispatchActionConfiguration::builder()
///     .trigger_action(TriggerAction::OnEnterPressed)
///     .trigger_action(TriggerAction::OnCharacterEntered)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "trigger_actions_on": [
///         "on_enter_pressed",
///         "on_character_entered"
///     ]
/// });
///
/// let json = serde_json::to_value(config).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let config = DispatchActionConfiguration::builder()
///     .trigger_actions_on(vec![])
///     .build();
/// assert!(config.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct DispatchActionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "trigger_action", validate("list::not_empty"))]
    pub(crate) trigger_actions_on: Option<Vec<TriggerAction>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DispatchActionConfiguration {
            trigger_actions_on: Some(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ]),
        };

        let config = DispatchActionConfiguration::builder()
            .set_trigger_actions_on(Some(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ]))
            .build()
            .unwrap();

        assert_eq!(config, expected);

        let config = DispatchActionConfiguration::builder()
            .trigger_actions_on(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ])
            .build()
            .unwrap();

        assert_eq!(config, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = DispatchActionConfiguration {
            trigger_actions_on: Some(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ]),
        };

        let config = DispatchActionConfiguration::builder()
            .trigger_action(TriggerAction::OnEnterPressed)
            .trigger_action(TriggerAction::OnCharacterEntered)
            .build()
            .unwrap();

        assert_eq!(config, expected);
    }

    #[test]
    fn it_requires_trigger_actions_on_field_not_empty() {
        let err = DispatchActionConfiguration::builder()
            .trigger_actions_on(vec![])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DispatchActionConfiguration");

        let errors = err.field("trigger_actions_on");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }
}
