use super::{MrkdwnText, PlainText, Text};
use serde::Serialize;

/// TextObject is a trait any text object representations must satisfy.
pub trait TextObject {
    fn text(&self) -> Option<&String>;
}

impl TextObject for Text {
    fn text(&self) -> Option<&String> {
        match self {
            Self::Plain(t) => t.text(),
            Self::Mrkdwn(t) => t.text(),
        }
    }
}

impl TextObject for MrkdwnText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

impl TextObject for PlainText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

/// Type of conversation to set into [Conversation filter object](https://docs.slack.dev/reference/block-kit/composition-objects/conversation-filter-object)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Conversation {
    Im,
    Mpim,
    Private,
    Public,
}
