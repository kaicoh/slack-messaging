use super::{MrkdwnText, PlainText, Text};

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
