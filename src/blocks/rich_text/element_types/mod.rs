/// Builders for rich text element types.
pub mod builders;
/// Additional types to create rich text element types.
pub mod types;

mod broadcast;
mod channel;
mod color;
mod date;
mod emoji;
mod link;
mod text;
mod user;
mod usergroup;

pub use broadcast::RichTextElementBroadcast;
pub use channel::RichTextElementChannel;
pub use color::RichTextElementColor;
pub use date::RichTextElementDate;
pub use emoji::RichTextElementEmoji;
pub use link::RichTextElementLink;
pub use text::RichTextElementText;
pub use user::RichTextElementUser;
pub use usergroup::RichTextElementUserGroup;

#[cfg(test)]
pub mod test_helpers {
    use super::types::*;

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
