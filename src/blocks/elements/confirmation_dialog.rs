use super::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ConfirmationDialog {
    title: Text,

    text: Text,

    confirm: Text,

    deny: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,
}

impl ConfirmationDialog {
    pub fn new<S, T, U, V>(title: S, text: T, confirm: U, deny: V) -> Self
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
        V: Into<String>,
    {
        Self {
            title: Text::plain(title),
            text: Text::plain(text),
            confirm: Text::plain(confirm),
            deny: Text::plain(deny),
            style: None,
        }
    }

    pub fn set_title(self, title: Text) -> Self {
        Self { title, ..self }
    }

    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    pub fn set_confirm(self, confirm: Text) -> Self {
        Self { confirm, ..self }
    }

    pub fn set_deny(self, deny: Text) -> Self {
        Self { deny, ..self }
    }

    pub fn set_primay(self) -> Self {
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
}
