use serde::Serialize;
use super::{ConfirmationDialog, Opt};

#[derive(Debug, Serialize)]
pub struct CheckboxGroup {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl CheckboxGroup {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "checkboxes",
            action_id: action_id.into(),
            options: vec![],
            initial_options: vec![],
            confirm: None,
            focus_on_load: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self { action_id: action_id.into(), ..self }
    }

    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    pub fn set_initial_options(self, initial_options: Vec<Opt>) -> Self {
        Self { initial_options, ..self }
    }

    pub fn push_initial_option(self, initial_option: Opt) -> Self {
        let Self { mut initial_options, .. } = self;
        initial_options.push(initial_option);
        Self { initial_options, ..self }
    }

    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }
}