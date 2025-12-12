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
mod workflow;

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
pub use workflow::Workflow;

#[cfg(test)]
pub mod test_helpers {
    use super::types::*;
    use super::*;

    pub fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }

    pub fn mrkdwn_text(text: impl Into<String>) -> MrkdwnText {
        MrkdwnText {
            text: Some(text.into()),
            verbatim: None,
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

    pub fn option_t(text: impl Into<String>, value: impl Into<String>) -> Opt<Text> {
        Opt {
            phantom: std::marker::PhantomData,
            text: Some(mrkdwn_text(text).into()),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }

    pub fn option_group(
        label: impl Into<String>,
        options: Vec<Opt<PlainText>>,
    ) -> OptGroup<PlainText> {
        OptGroup {
            label: Some(plain_text(label)),
            options: Some(options),
        }
    }

    pub fn input_param(name: impl Into<String>, value: impl Into<String>) -> InputParameter {
        InputParameter {
            name: Some(name.into()),
            value: Some(serde_json::Value::String(value.into())),
        }
    }

    pub fn trigger() -> Trigger {
        Trigger {
            url: Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz".into()),
            customizable_input_parameters: Some(vec![
                input_param("param_0", "value_0"),
                input_param("param_1", "value_1"),
            ]),
        }
    }

    pub fn confirm() -> ConfirmationDialog {
        ConfirmationDialog {
            title: Some(plain_text("Are you sure?")),
            text: Some(plain_text("Wouldn't you prefer a good game of _chess_?")),
            confirm: Some(plain_text("Do it")),
            deny: Some(plain_text("Stop, I've changed my mind!")),
            style: None,
        }
    }

    pub fn dispatch_action_config() -> DispatchActionConfiguration {
        DispatchActionConfiguration {
            trigger_actions_on: Some(vec![TriggerAction::OnEnterPressed]),
        }
    }

    pub fn slack_file() -> SlackFile {
        SlackFile {
            id: Some("F0123456".into()),
            url: None,
        }
    }

    pub fn conversation_filter() -> ConversationFilter {
        ConversationFilter {
            include: Some(vec![Conversation::Public]),
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
        }
    }

    pub fn workflow() -> Workflow {
        Workflow {
            trigger: Some(trigger()),
        }
    }
}
