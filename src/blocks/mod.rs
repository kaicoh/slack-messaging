/// Builder objects.
pub mod builders;

/// Objects from that the blocks are composed.
pub mod elements;

mod actions;
mod context;
mod divider;
mod file;
mod header;
mod image;
mod input;
/// Types from which Rich text composed of
pub mod rich_text;
mod section;
mod video;

use super::composition_objects;
use serde::Serialize;

pub use actions::{Actions, ActionsElement};
pub use context::{Context, ContextElement};
pub use divider::Divider;
pub use file::{File, FileSource};
pub use header::Header;
pub use image::Image;
pub use input::{Input, InputElement};
pub use rich_text::RichText;
pub use section::{Accessory, Section};
pub use video::Video;

/// Objects that can be set to blocks in [Message](crate::message::Message).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Block {
    /// [Actions block](https://api.slack.com/reference/block-kit/blocks#actions) representation
    Actions(Box<Actions>),

    /// [Context block](https://api.slack.com/reference/block-kit/blocks#context) representation
    Context(Box<Context>),

    /// [Divider block](https://api.slack.com/reference/block-kit/blocks#divider) representation
    Divider(Box<Divider>),

    /// [File block](https://api.slack.com/reference/block-kit/blocks#file) representation
    File(Box<File>),

    /// [Header block](https://api.slack.com/reference/block-kit/blocks#header) representation
    Header(Box<Header>),

    /// [Image block](https://api.slack.com/reference/block-kit/blocks#image) representation
    Image(Box<Image>),

    /// [Input block](https://api.slack.com/reference/block-kit/blocks#input) representation
    Input(Box<Input>),

    /// [Rich text block](https://api.slack.com/reference/block-kit/blocks#rich_text) representation
    RichText(Box<RichText>),

    /// [Section block](https://api.slack.com/reference/block-kit/blocks#section) representation
    Section(Box<Section>),

    /// [Video block](https://api.slack.com/reference/block-kit/blocks#video) representation
    Video(Box<Video>),
}

macro_rules! block_from {
    ($($ty:ident),*) => {
        $(
            impl From<$ty> for Block {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

block_from! {
    Actions,
    Context,
    Divider,
    File,
    Header,
    Image,
    Input,
    RichText,
    Section,
    Video
}
