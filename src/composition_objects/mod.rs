use super::{Builder, error, validators, value};

/// Builder objects for Composition objects.
pub mod builders;

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
mod workflow;

pub use confirmation_dialog::ConfirmationDialog;
pub use conversation_filter::{Conversation, ConversationFilter};
pub use dispatch_action_configuration::{DispatchActionConfiguration, TriggerAction};
pub use markdown_text::MrkdwnText;
pub use option::{Opt, TextInOption};
pub use option_group::OptGroup;
pub use plain_text::PlainText;
pub use slack_file::SlackFile;
pub use text::{Text, TextObject};
pub use trigger::{InputParameter, Trigger};
pub use workflow::Workflow;

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;

    pub(crate) fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }

    pub(crate) fn mrkdwn(text: impl Into<String>) -> MrkdwnText {
        MrkdwnText {
            text: Some(text.into()),
            verbatim: None,
        }
    }

    pub(crate) fn option_plain(
        text: impl Into<String>,
        value: impl Into<String>,
    ) -> Opt<PlainText> {
        Opt {
            text: Some(plain_text(text)),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }

    pub(crate) fn option_text(text: impl Into<String>, value: impl Into<String>) -> Opt<Text> {
        Opt {
            text: Some(mrkdwn(text).into()),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }

    pub(crate) fn confirm() -> ConfirmationDialog {
        ConfirmationDialog {
            title: Some(plain_text("Are you sure?")),
            text: Some(plain_text("Wouldn't you prefer a good game of _chess_?")),
            confirm: Some(plain_text("Do it")),
            deny: Some(plain_text("Stop, I've changed my mind!")),
            style: None,
        }
    }

    pub(crate) fn dispatch_action_config() -> DispatchActionConfiguration {
        DispatchActionConfiguration {
            trigger_actions_on: vec![
                TriggerAction::OnEnterPressed,
                TriggerAction::OnCharacterEntered,
            ],
        }
    }

    pub(crate) fn trigger() -> Trigger {
        Trigger {
            url: Some("foo".into()),
            customizable_input_parameters: vec![InputParameter {
                name: Some("bar".into()),
                value: Some("baz".into()),
            }],
        }
    }

    pub(crate) fn slack_file() -> SlackFile {
        SlackFile {
            id: Some("12345".into()),
            url: None,
        }
    }
}
