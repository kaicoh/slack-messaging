use serde::Serialize;

/// Interaction type to set into [Dispatch action configuration](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    /// Represents `on_enter_pressed`.
    OnEnterPressed,

    /// Represents `on_character_entered`.
    OnCharacterEntered,
}

/// [Dispatch action configuration](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object)
/// representation.
///
/// The Builder returns
/// [`DispatchActionConfigurationError`](crate::composition_objects::builders::DispatchActionConfigurationError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
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
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DispatchActionConfiguration {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) trigger_actions_on: Vec<TriggerAction>,
}
