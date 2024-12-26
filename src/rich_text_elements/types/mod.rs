use serde::Serialize;

/// Builder objects for Rich text element types.
pub mod builders;

mod broadcast;
mod channel;
mod color;
mod date;
mod emoji;
mod link;
mod styles;
mod text;
mod user;
mod usergroup;

pub use broadcast::{BroadcastRange, RichTextElementTypeBroadcast};
pub use channel::RichTextElementTypeChannel;
pub use color::RichTextElementTypeColor;
pub use date::RichTextElementTypeDate;
pub use emoji::RichTextElementTypeEmoji;
pub use link::RichTextElementTypeLink;
pub use styles::{CodableStyle, HighlightableStyle};
pub use text::RichTextElementTypeText;
pub use user::RichTextElementTypeUser;
pub use usergroup::RichTextElementTypeUserGroup;

/// [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
/// representation.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum RichTextElementType {
    Broadcast(Box<RichTextElementTypeBroadcast>),
    Channel(Box<RichTextElementTypeChannel>),
    Color(Box<RichTextElementTypeColor>),
    Date(Box<RichTextElementTypeDate>),
    Emoji(Box<RichTextElementTypeEmoji>),
    Link(Box<RichTextElementTypeLink>),
    Text(Box<RichTextElementTypeText>),
    User(Box<RichTextElementTypeUser>),
    UserGroup(Box<RichTextElementTypeUserGroup>),
}

impl From<RichTextElementTypeBroadcast> for RichTextElementType {
    fn from(value: RichTextElementTypeBroadcast) -> Self {
        Self::Broadcast(Box::new(value))
    }
}

impl From<RichTextElementTypeChannel> for RichTextElementType {
    fn from(value: RichTextElementTypeChannel) -> Self {
        Self::Channel(Box::new(value))
    }
}

impl From<RichTextElementTypeColor> for RichTextElementType {
    fn from(value: RichTextElementTypeColor) -> Self {
        Self::Color(Box::new(value))
    }
}

impl From<RichTextElementTypeDate> for RichTextElementType {
    fn from(value: RichTextElementTypeDate) -> Self {
        Self::Date(Box::new(value))
    }
}

impl From<RichTextElementTypeEmoji> for RichTextElementType {
    fn from(value: RichTextElementTypeEmoji) -> Self {
        Self::Emoji(Box::new(value))
    }
}

impl From<RichTextElementTypeLink> for RichTextElementType {
    fn from(value: RichTextElementTypeLink) -> Self {
        Self::Link(Box::new(value))
    }
}

impl From<RichTextElementTypeText> for RichTextElementType {
    fn from(value: RichTextElementTypeText) -> Self {
        Self::Text(Box::new(value))
    }
}

impl From<RichTextElementTypeUser> for RichTextElementType {
    fn from(value: RichTextElementTypeUser) -> Self {
        Self::User(Box::new(value))
    }
}

impl From<RichTextElementTypeUserGroup> for RichTextElementType {
    fn from(value: RichTextElementTypeUserGroup) -> Self {
        Self::UserGroup(Box::new(value))
    }
}
