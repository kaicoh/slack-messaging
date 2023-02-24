use super::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MultiSelectExternals {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl MultiSelectExternals {
    pub fn new<T: Into<String>>(action_id: T) -> Self {
        Self {
            kind: "multi_external_select",
            action_id: action_id.into(),
            min_query_length: None,
            initial_options: vec![],
            confirm: None,
            max_selected_items: None,
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

    pub fn set_initial_options(self, initial_options: Vec<Opt>) -> Self {
        Self {
            initial_options,
            ..self
        }
    }

    pub fn push_initial_option(self, initial_option: Opt) -> Self {
        let Self {
            mut initial_options,
            ..
        } = self;
        initial_options.push(initial_option);
        Self {
            initial_options,
            ..self
        }
    }

    pub fn set_min_query_length<T: Into<i64>>(self, length: T) -> Self {
        Self {
            min_query_length: Some(length.into()),
            ..self
        }
    }

    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    pub fn set_max_selected_items<T: Into<i64>>(self, items: T) -> Self {
        Self {
            max_selected_items: Some(items.into()),
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
