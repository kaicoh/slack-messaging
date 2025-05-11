/// Builder objects for Blocks.
pub mod builders;

/// Objects from which blocks are composed.
pub mod elements;

mod actions;
mod context;
mod divider;
mod file;
mod header;
mod image;
mod input;
mod rich_text;
mod section;
mod video;

use super::{composition_objects, rich_text_elements};
use serde::Serialize;

pub use actions::{Actions, ActionsElement};
pub use context::{Context, ContextElement};
pub use divider::Divider;
pub use file::{File, FileSource};
pub use header::Header;
pub use image::Image;
pub use input::{Input, InputElement};
pub use rich_text::{RichText, RichTextElement};
pub use section::{Accessory, Section};
pub use video::Video;

/// Objects that can be set to blocks in [Message](crate::message::Message).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Block {
    /// [Actions block](https://docs.slack.dev/reference/block-kit/blocks/actions-block) representation
    Actions(Box<Actions>),

    /// [Context block](https://docs.slack.dev/reference/block-kit/blocks/context-block) representation
    Context(Box<Context>),

    /// [Divider block](https://docs.slack.dev/reference/block-kit/blocks/divider-block) representation
    Divider(Box<Divider>),

    /// [File block](https://docs.slack.dev/reference/block-kit/blocks/file-block) representation
    File(Box<File>),

    /// [Header block](https://docs.slack.dev/reference/block-kit/blocks/header-block) representation
    Header(Box<Header>),

    /// [Image block](https://docs.slack.dev/reference/block-kit/blocks/image-block) representation
    Image(Box<Image>),

    /// [Input block](https://docs.slack.dev/reference/block-kit/blocks/input-block) representation
    Input(Box<Input>),

    /// [Rich text block](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block) representation
    RichText(Box<RichText>),

    /// [Section block](https://docs.slack.dev/reference/block-kit/blocks/section-block) representation
    Section(Box<Section>),

    /// [Video block](https://docs.slack.dev/reference/block-kit/blocks/video-block) representation
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
