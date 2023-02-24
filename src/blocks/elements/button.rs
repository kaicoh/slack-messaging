use super::{ConfirmationDialog, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    kind: &'static str,

    text: Text,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_label: Option<String>,
}

impl Button {
    pub fn new<S, T>(text: S, action_id: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            kind: "button",
            text: Text::plain(text),
            action_id: action_id.into(),
            url: None,
            value: None,
            style: None,
            confirm: None,
            accessibility_label: None,
        }
    }

    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    pub fn set_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }

    pub fn set_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: Some(value.into()),
            ..self
        }
    }

    pub fn set_primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    pub fn set_danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }

    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    pub fn set_accessibility_label<T: Into<String>>(self, label: T) -> Self {
        Self {
            accessibility_label: Some(label.into()),
            ..self
        }
    }
}
