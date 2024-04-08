use super::{DispatchActionConfiguration, TriggerAction};

impl DispatchActionConfiguration {
    /// Construct a [`DispatchActionConfigurationBuilder`].
    pub fn builder() -> DispatchActionConfigurationBuilder {
        DispatchActionConfigurationBuilder::default()
    }
}

/// Builder for [`DispatchActionConfiguration`] object.
#[derive(Debug, Default)]
pub struct DispatchActionConfigurationBuilder {
    trigger_actions_on: Vec<TriggerAction>,
}

impl DispatchActionConfigurationBuilder {
    /// Set trigger_actions_on field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let config = DispatchActionConfiguration::builder()
    ///     .set_trigger_actions(
    ///         vec![
    ///             TriggerAction::OnEnterPressed,
    ///             TriggerAction::OnCharacterEntered,
    ///         ]
    ///     )
    ///     .build();
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
    /// ```
    pub fn set_trigger_actions(self, actions: Vec<TriggerAction>) -> Self {
        Self {
            trigger_actions_on: actions,
        }
    }

    /// Add trigger_action to trigger_actions_on field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{DispatchActionConfiguration, TriggerAction};
    /// let config = DispatchActionConfiguration::builder()
    ///     .trigger_action(TriggerAction::OnEnterPressed)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "trigger_actions_on": [
    ///         "on_enter_pressed"
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(config).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn trigger_action(self, action: TriggerAction) -> Self {
        let Self {
            mut trigger_actions_on,
        } = self;
        trigger_actions_on.push(action);
        Self { trigger_actions_on }
    }

    /// Build a [`DispatchActionConfiguration`] object.
    pub fn build(self) -> DispatchActionConfiguration {
        DispatchActionConfiguration {
            trigger_actions_on: self.trigger_actions_on,
        }
    }

    /// Get trigger_actions value.
    pub fn get_trigger_actions(&self) -> &[TriggerAction] {
        &self.trigger_actions_on
    }
}
