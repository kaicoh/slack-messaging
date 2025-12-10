/// builder objects for composition objects.
pub mod builders;

/// Additional types to create composition objects.
pub mod types;

mod confirmation_dialog;
mod conversation_filter;
mod dispatch_action_configuration;
mod markdown_text;
mod option;
mod option_group;
mod plain_text;
mod slack_file;
mod text;
mod trigger;

pub use confirmation_dialog::ConfirmationDialog;
pub use conversation_filter::ConversationFilter;
pub use dispatch_action_configuration::DispatchActionConfiguration;
pub use markdown_text::MrkdwnText;
pub use option::Opt;
pub use option_group::OptGroup;
pub use plain_text::PlainText;
pub use slack_file::SlackFile;
pub use text::Text;
pub use trigger::Trigger;

#[cfg(test)]
mod test_helpers {
    use super::types::*;
    use super::*;

    pub fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }

    pub fn option(text: impl Into<String>, value: impl Into<String>) -> Opt<PlainText> {
        Opt {
            phantom: std::marker::PhantomData,
            text: Some(plain_text(text)),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }

    pub fn input_param(name: impl Into<String>, value: impl Into<String>) -> InputParameter {
        InputParameter {
            name: Some(name.into()),
            value: Some(serde_json::Value::String(value.into())),
        }
    }
}
