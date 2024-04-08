/// Builder objects for Composition objects.
pub mod builders;

mod confirmation_dialog;
mod conversation_filter;
mod dispatch_action_configuration;
mod input_parameter;
mod option;
mod option_group;
mod slack_file;
mod text;
mod trigger;
mod workflow;

use text::TYPE_MRKDWN;
use text::TYPE_PLAIN;

pub use confirmation_dialog::ConfirmationDialog;
pub use conversation_filter::{Conversation, ConversationFilter};
pub use dispatch_action_configuration::{DispatchActionConfiguration, TriggerAction};
pub use input_parameter::InputParameter;
pub use option::Opt;
pub use option_group::OptGroup;
pub use slack_file::SlackFile;
pub use text::Text;
pub use trigger::Trigger;
pub use workflow::Workflow;
