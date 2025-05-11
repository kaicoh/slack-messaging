//! ```
//! use slack_messaging::blocks::rich_text::elements::{
//!     RichTextElementTypeText, RichTextList, RichTextListStyle, RichTextSection,
//! };
//!
//! let list = RichTextList::builder()
//!     .style(RichTextListStyle::Bullet)
//!     .element(
//!         RichTextSection::builder()
//!             .element(
//!                 RichTextElementTypeText::builder()
//!                     .text("Huddles")
//!                     .build()
//!             )
//!             .build()
//!     )
//!     .element(
//!         RichTextSection::builder()
//!             .element(
//!                 RichTextElementTypeText::builder()
//!                     .text("Canvas")
//!                     .build()
//!             )
//!             .build()
//!     )
//!     .build();
//!
//! let expected = serde_json::json!({
//!     "type": "rich_text_list",
//!     "style": "bullet",
//!     "elements": [
//!         {
//!             "type": "rich_text_section",
//!             "elements": [
//!                 {
//!                     "type": "text",
//!                     "text": "Huddles",
//!                 }
//!             ]
//!         },
//!         {
//!             "type": "rich_text_section",
//!             "elements": [
//!                 {
//!                     "type": "text",
//!                     "text": "Canvas",
//!                 }
//!             ]
//!         }
//!     ]
//! });
//!
//! let json = serde_json::to_value(list).unwrap();
//!
//! assert_eq!(json, expected);
//! ```

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
    BroadcastRange, CodableStyle, HighlightableStyle, RichTextElementType,
    RichTextElementTypeBroadcast, RichTextElementTypeChannel, RichTextElementTypeColor,
    RichTextElementTypeDate, RichTextElementTypeEmoji, RichTextElementTypeLink,
    RichTextElementTypeText, RichTextElementTypeUser, RichTextElementTypeUserGroup,
};
