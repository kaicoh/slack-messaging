use super::Opt;
use super::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct OptGroup {
    label: Text,
    options: Vec<Opt>,
}

impl OptGroup {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            label: Text::plain(label),
            options: vec![],
        }
    }

    pub fn set_label(self, label: Text) -> Self {
        Self { label, ..self }
    }

    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }
}
