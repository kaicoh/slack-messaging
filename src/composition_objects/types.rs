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

/// Interaction type to set into [Dispatch action configuration](https://docs.slack.dev/reference/block-kit/composition-objects/dispatch-action-configuration-object)
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TriggerAction {
    /// Represents `on_enter_pressed`.
    OnEnterPressed,

    /// Represents `on_character_entered`.
    OnCharacterEntered,
}

/// TextInOption is a trait that can be set to text and desciption field
/// of [`Opt`](crate::composition_objects::Opt) object
pub trait TextInOption: TextObject {}

impl TextInOption for Text {}
impl TextInOption for PlainText {}

/// Phantom type to control url field of [`Opt`](crate::composition_objects::Opt). By default, this type is used,
/// and the url field is unavailable.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlUnavailable;

/// Phantom type to control url field of [`Opt`](crate::composition_objects::Opt). Using this type, the url field
/// is available.
#[derive(Debug, Clone, PartialEq)]
pub struct UrlAvailable;
