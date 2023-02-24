use super::{ConfirmationDialog, Opt};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OverflowMenu {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenu {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "overflow",
            action_id: action_id.into(),
            options: vec![],
            confirm: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }
}
