use super::{ConfirmationDialog, Filter, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SelectConversations {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    response_url_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<Filter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl SelectConversations {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "conversations_select",
            action_id: action_id.into(),
            initial_conversation: None,
            default_to_current_conversation: None,
            confirm: None,
            response_url_enabled: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    pub fn set_initial_conversation<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_conversation: Some(value.into()),
            ..self
        }
    }

    pub fn set_default_to_current_conversation(self, current_conversation: bool) -> Self {
        Self {
            default_to_current_conversation: Some(current_conversation),
            ..self
        }
    }

    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    pub fn set_response_url_enabled(self, enabled: bool) -> Self {
        Self {
            response_url_enabled: Some(enabled),
            ..self
        }
    }

    pub fn response_url_enabled(self) -> Self {
        self.set_response_url_enabled(true)
    }

    pub fn response_url_disabled(self) -> Self {
        self.set_response_url_enabled(false)
    }

    pub fn set_filter(self, filter: Filter) -> Self {
        Self {
            filter: Some(filter),
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
}
