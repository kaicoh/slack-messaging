use serde::Serialize;
use super::ConfirmationDialog;

#[derive(Debug, Serialize)]
pub struct DatetimePicker {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_date_time: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl DatetimePicker {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "datetimepicker",
            action_id: action_id.into(),
            initial_date_time: None,
            confirm: None,
            focus_on_load: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self { action_id: action_id.into(), ..self }
    }

    pub fn set_initial_date_time<T: Into<i64>>(self, datetime: T) -> Self {
        Self {
            initial_date_time: Some(datetime.into()),
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
}
