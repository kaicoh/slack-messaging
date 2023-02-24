use serde::Serialize;
use super::{DispatchActionConfiguration, Text};

#[derive(Debug, Serialize)]
pub struct PlainTextInput {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    multiline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl PlainTextInput {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "plain_text_input",
            action_id: action_id.into(),
            initial_value: None,
            multiline: None,
            min_length: None,
            max_length: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }

    pub fn set_action_id<T: Into<String>>(self, value: T) -> Self {
        Self {
            action_id: value.into(),
            ..self
        }
    }

    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    pub fn set_multiline(self, value: bool) -> Self {
        Self {
            multiline: Some(value),
            ..self
        }
    }

    pub fn multiline(self) -> Self {
        self.set_multiline(true)
    }

    pub fn set_min_length<T: Into<i64>>(self, value: T) -> Self {
        Self {
            min_length: Some(value.into()),
            ..self
        }
    }

    pub fn set_max_length<T: Into<i64>>(self, value: T) -> Self {
        Self {
            max_length: Some(value.into()),
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
