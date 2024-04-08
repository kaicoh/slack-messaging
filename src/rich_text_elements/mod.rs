/// Builder objects for Rich text elements.
pub mod builders;

mod list;
mod preformatted;
mod quote;
mod section;

/// Types Rich text elements composed of.
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
