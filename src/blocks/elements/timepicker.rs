use super::{ConfirmationDialog, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TimePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_time: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
}

impl TimePicker {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "timepicker",
            action_id: action_id.into(),
            initial_time: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
            timezone: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    pub fn set_initial_time<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_time: Some(value.into()),
            ..self
        }
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

    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }

    pub fn set_timezone<T: Into<String>>(self, value: T) -> Self {
        Self {
            timezone: Some(value.into()),
            ..self
        }
    }
}
