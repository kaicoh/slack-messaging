use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    OnEnterPressed,
    OnCharacterEntered,
}

#[derive(Debug, Default, Serialize)]
pub struct DispatchActionConfiguration {
    trigger_actions_on: Vec<TriggerAction>,
}

impl DispatchActionConfiguration {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_trigger_action(self, action: TriggerAction) -> Self {
        let Self { mut trigger_actions_on } = self;
        trigger_actions_on.push(action);
        Self { trigger_actions_on }
    }
}
