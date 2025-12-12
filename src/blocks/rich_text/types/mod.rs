use paste::paste;
use serde::Serialize;

/// Builders for rich text element types.
pub mod builders;

mod broadcast;
mod channel;
mod color;
mod date;
mod emoji;
mod link;
mod style;
mod text;
mod user;
mod usergroup;

pub use broadcast::{BroadcastRange, RichTextElementBroadcast};
pub use channel::RichTextElementChannel;
pub use color::RichTextElementColor;
pub use date::RichTextElementDate;
pub use emoji::RichTextElementEmoji;
pub use link::RichTextElementLink;
pub use style::{RichTextStyle, StyleTypeFour, StyleTypeSix};
pub use text::RichTextElementText;
pub use user::RichTextElementUser;
pub use usergroup::RichTextElementUserGroup;

/// [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
/// representation.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum RichTextElementType {
    /// [broadcast element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#broadcast-element-type)
    Broadcast(Box<RichTextElementBroadcast>),

    /// [channel element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#channel-element-type)
    Channel(Box<RichTextElementChannel>),

    /// [color element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#color-element-type)
    Color(Box<RichTextElementColor>),

    /// [date element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#date-element-type)
    Date(Box<RichTextElementDate>),

    /// [emoji element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#emoji-element-type)
    Emoji(Box<RichTextElementEmoji>),

    /// [link element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block/#link-element-type)
    Link(Box<RichTextElementLink>),

    /// [text element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#text-element-type)
    Text(Box<RichTextElementText>),

    /// [user element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-element-type)
    User(Box<RichTextElementUser>),

    /// [usergroup element type](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block#user-group-element-type)
    UserGroup(Box<RichTextElementUserGroup>),
}

macro_rules! impl_element_type {
    ($($var:tt,)*) => {
        paste! {
            $(
                impl From<[<RichTextElement $var>]> for RichTextElementType {
                    fn from(value: [<RichTextElement $var>]) -> Self {
                        Self::$var(Box::new(value))
                    }
                }
            )*
        }
    };
}

impl_element_type! {
    Broadcast,
    Channel,
    Color,
    Date,
    Emoji,
    Link,
    Text,
    User,
    UserGroup,
}

#[cfg(test)]
pub mod test_helpers {
    use super::*;

    pub fn text(text: impl Into<String>) -> RichTextElementText {
        RichTextElementText {
            text: Some(text.into()),
            style: None,
        }
    }

    pub fn el_text(inner_text: impl Into<String>) -> RichTextElementType {
        text(inner_text).into()
    }

    pub fn emoji(name: impl Into<String>) -> RichTextElementEmoji {
        RichTextElementEmoji {
            name: Some(name.into()),
            unicode: None,
        }
    }

    pub fn el_emoji(name: impl Into<String>) -> RichTextElementType {
        emoji(name).into()
    }

    pub fn style_six() -> RichTextStyle<StyleTypeSix> {
        RichTextStyle {
            phantom: std::marker::PhantomData,
            bold: Some(true),
            italic: None,
            strike: None,
            highlight: None,
            client_highlight: Some(true),
            unlink: None,
            code: None,
        }
    }

    pub fn style_four() -> RichTextStyle<StyleTypeFour> {
        RichTextStyle {
            phantom: std::marker::PhantomData,
            bold: None,
            italic: Some(true),
            strike: None,
            highlight: None,
            client_highlight: None,
            unlink: None,
            code: Some(true),
        }
    }
}
