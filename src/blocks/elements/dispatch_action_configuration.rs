use serde::Serialize;

/// Interaction type to set into [Dispatch action configuration](https://api.slack.com/reference/block-kit/composition-objects#dispatch_action_config)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    /// Represents `on_enter_pressed`.
    OnEnterPressed,

    /// Represents `on_character_entered`.
    OnCharacterEntered,
}

/// [Dispatch action configuration](https://api.slack.com/reference/block-kit/composition-objects#dispatch_action_config)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{DispatchActionConfiguration, TriggerAction};
/// use serde_json::json;
///
/// let config = DispatchActionConfiguration::new()
///     .push_trigger_action(TriggerAction::OnEnterPressed)
///     .push_trigger_action(TriggerAction::OnCharacterEntered);
///
/// let expected = json!({
///     "trigger_actions_on": [
///         "on_enter_pressed",
///         "on_character_entered"
///     ]
/// });
///
/// let config_json = serde_json::to_value(config).unwrap();
///
/// assert_eq!(config_json, expected);
/// ```
#[derive(Debug, Default, Clone, Serialize)]
pub struct DispatchActionConfiguration {
    trigger_actions_on: Vec<TriggerAction>,
}

impl DispatchActionConfiguration {
    /// Constructs a Dispatch action configuration.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::DispatchActionConfiguration;
    /// use serde_json::json;
    ///
    /// let config = DispatchActionConfiguration::new();
    ///
    /// let expected = json!({
    ///     "trigger_actions_on": []
    /// });
    ///
    /// let config_json = serde_json::to_value(config).unwrap();
    ///
    /// assert_eq!(config_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets trigger_actions_on field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{DispatchActionConfiguration, TriggerAction};
    /// use serde_json::json;
    ///
    /// let config = DispatchActionConfiguration::new()
    ///     .set_trigger_actions(
    ///         vec![
    ///             TriggerAction::OnEnterPressed,
    ///             TriggerAction::OnCharacterEntered,
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "trigger_actions_on": [
    ///         "on_enter_pressed",
    ///         "on_character_entered"
    ///     ]
    /// });
    ///
    /// let config_json = serde_json::to_value(config).unwrap();
    ///
    /// assert_eq!(config_json, expected);
    /// ```
    pub fn set_trigger_actions(self, actions: Vec<TriggerAction>) -> Self {
        Self {
            trigger_actions_on: actions,
        }
    }

    /// Adds trigger_action to trigger_actions_on field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{DispatchActionConfiguration, TriggerAction};
    /// use serde_json::json;
    ///
    /// let config = DispatchActionConfiguration::new()
    ///     .push_trigger_action(TriggerAction::OnEnterPressed);
    ///
    /// let expected = json!({
    ///     "trigger_actions_on": [
    ///         "on_enter_pressed"
    ///     ]
    /// });
    ///
    /// let config_json = serde_json::to_value(config).unwrap();
    ///
    /// assert_eq!(config_json, expected);
    /// ```
    pub fn push_trigger_action(self, action: TriggerAction) -> Self {
        let Self {
            mut trigger_actions_on,
        } = self;
        trigger_actions_on.push(action);
        Self { trigger_actions_on }
    }
}
