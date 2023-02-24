use serde::Serialize;
use super::{DispatchActionConfiguration, Text};

#[derive(Debug, Serialize)]
pub struct UrlInput {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl UrlInput {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "url_text_input",
            action_id: action_id.into(),
            initial_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self { action_id: action_id.into(), ..self }
    }

    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    pub fn set_dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        Self {
            dispatch_action_config: Some(config),
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
