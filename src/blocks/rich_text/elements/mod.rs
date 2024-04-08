use serde::Serialize;

/// Builder objects.
pub mod builders;

pub mod list;
pub mod preformatted;
pub mod quote;
pub mod section;
/// Types rich text composed of.
pub mod types;

pub use list::{RichTextList, RichTextListStyle};
pub use preformatted::RichTextPreformatted;
pub use quote::RichTextQuote;
pub use section::RichTextSection;
pub use types::{
    CodableStyle, HighlightableStyle, RichTextElementType, RichTextElementTypeChannel,
    RichTextElementTypeEmoji, RichTextElementTypeLink, RichTextElementTypeText,
    RichTextElementTypeUser, RichTextElementTypeUserGroup,
};

/// Rich text object.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichTextElement {
    Section(Box<RichTextSection>),
    List(Box<RichTextList>),
    Preformatted(Box<RichTextPreformatted>),
    Quote(Box<RichTextQuote>),
}

impl From<RichTextSection> for RichTextElement {
    fn from(value: RichTextSection) -> Self {
        Self::Section(Box::new(value))
    }
}

impl From<RichTextList> for RichTextElement {
    fn from(value: RichTextList) -> Self {
        Self::List(Box::new(value))
    }
}

impl From<RichTextPreformatted> for RichTextElement {
    fn from(value: RichTextPreformatted) -> Self {
        Self::Preformatted(Box::new(value))
    }
}

impl From<RichTextQuote> for RichTextElement {
    fn from(value: RichTextQuote) -> Self {
        Self::Quote(Box::new(value))
    }
}
