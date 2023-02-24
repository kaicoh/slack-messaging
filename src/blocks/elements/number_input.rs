use super::{DispatchActionConfiguration, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NumberInput {
    #[serde(rename = "type")]
    kind: &'static str,

    is_decimal_allowed: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for NumberInput {
    fn default() -> Self {
        Self {
            kind: "number_input",
            is_decimal_allowed: false,
            action_id: None,
            initial_value: None,
            min_value: None,
            max_value: None,
            dispatch_action_config: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl NumberInput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_is_decimal_allowed(self, is_decimal_allowed: bool) -> Self {
        Self {
            is_decimal_allowed,
            ..self
        }
    }

    pub fn decimal_allowed(self) -> Self {
        self.set_is_decimal_allowed(true)
    }

    pub fn decimal_disallowed(self) -> Self {
        self.set_is_decimal_allowed(false)
    }

    pub fn set_action_id<T: Into<String>>(self, value: T) -> Self {
        Self {
            action_id: Some(value.into()),
            ..self
        }
    }

    pub fn set_initial_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            initial_value: Some(value.into()),
            ..self
        }
    }

    pub fn set_min_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            min_value: Some(value.into()),
            ..self
        }
    }

    pub fn set_max_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            max_value: Some(value.into()),
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
