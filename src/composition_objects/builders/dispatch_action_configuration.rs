use super::error::ValidationError;
use super::validators;
use super::value::Value;
use super::{Builder, DispatchActionConfiguration, TriggerAction};

use std::error::Error;
use std::fmt;

impl DispatchActionConfiguration {
    /// Construct a [`DispatchActionConfigurationBuilder`].
    pub fn builder() -> DispatchActionConfigurationBuilder {
        DispatchActionConfigurationBuilder::default()
    }
}

/// Error while building [`DispatchActionConfiguration`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DispatchActionConfigurationError {
    /// errors of trigger_actions_on field
    pub trigger_actions_on: Vec<ValidationError>,
}

impl fmt::Display for DispatchActionConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DispatchActionConfigurationError {{ trigger_actions_on: {:?} }}",
            self.trigger_actions_on
        )
    }
}

impl Error for DispatchActionConfigurationError {}

/// Builder for [`DispatchActionConfiguration`] object.
#[derive(Debug)]
pub struct DispatchActionConfigurationBuilder {
    trigger_actions_on: Value<Vec<TriggerAction>>,
}

impl Default for DispatchActionConfigurationBuilder {
    fn default() -> Self {
        DispatchActionConfigurationBuilder {
            trigger_actions_on: new_trigger_actions_on(None),
        }
    }
}

impl Builder for DispatchActionConfigurationBuilder {
    type Target = DispatchActionConfiguration;
    type Error = DispatchActionConfigurationError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self { trigger_actions_on } = self;

        if trigger_actions_on.has_errors() {
            Err(DispatchActionConfigurationError {
                trigger_actions_on: trigger_actions_on.errors,
            })
        } else {
            Ok(DispatchActionConfiguration {
                trigger_actions_on: trigger_actions_on.inner.unwrap_or_default(),
            })
        }
    }
}

impl DispatchActionConfigurationBuilder {
    /// get trigger_actions_on field value
    pub fn get_trigger_actions_on(&self) -> Option<&[TriggerAction]> {
        self.trigger_actions_on.inner_ref().map(|v| v.as_ref())
    }

    /// set trigger_actions_on field value
    pub fn set_trigger_actions_on(self, values: Option<Vec<TriggerAction>>) -> Self {
        Self {
            trigger_actions_on: new_trigger_actions_on(values),
        }
    }

    /// set trigger_actions_on field value
    pub fn trigger_actions_on(self, values: Vec<TriggerAction>) -> Self {
        self.set_trigger_actions_on(Some(values))
    }

    /// add value to trigger_actions_on field
    pub fn trigger_action(mut self, value: TriggerAction) -> Self {
        let mut list = self.trigger_actions_on.take_inner().unwrap_or_default();
        list.push(value);
        self.trigger_actions_on(list)
    }
}

fn new_trigger_actions_on(value: Option<Vec<TriggerAction>>) -> Value<Vec<TriggerAction>> {
    pipe! { Value::new(value) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let val = DispatchActionConfiguration::builder()
            .set_trigger_actions_on(Some(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ]))
            .build()
            .unwrap();

        let expected = DispatchActionConfiguration {
            trigger_actions_on: vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ],
        };
        assert_eq!(val, expected);

        let val = DispatchActionConfiguration::builder()
            .trigger_actions_on(vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_has_additional_setter_for_set_trigger_actions_on_field() {
        let val = DispatchActionConfiguration::builder()
            .trigger_action(TriggerAction::OnEnterPressed)
            .trigger_action(TriggerAction::OnCharacterEntered)
            .build()
            .unwrap();

        let expected = DispatchActionConfiguration {
            trigger_actions_on: vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ],
        };
        assert_eq!(val, expected);
    }
}
