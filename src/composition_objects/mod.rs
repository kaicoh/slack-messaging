/// builder objects for composition objects.
pub mod builders;

/// Additional types to create composition objects.
pub mod types;

mod confirmation_dialog;
mod conversation_filter;
mod dispatch_action_configuration;
mod markdown_text;
mod plain_text;
mod text;

pub use confirmation_dialog::ConfirmationDialog;
pub use conversation_filter::ConversationFilter;
pub use dispatch_action_configuration::DispatchActionConfiguration;
pub use markdown_text::MrkdwnText;
pub use plain_text::PlainText;
pub use text::Text;

#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
