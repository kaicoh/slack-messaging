/// Objects from that the blocks are composed.
pub mod elements;

mod actions;
mod context;
mod divider;
mod header;
mod image;
mod input;
mod section;
mod video;

use serde::Serialize;

pub use actions::{Actions, ActionsElement};
pub use context::{Context, ContextElement};
pub use divider::Divider;
pub use header::Header;
pub use image::Image;
pub use input::{Input, InputElement};
pub use section::{Accessory, Section};
pub use video::Video;

/// Objects that can be set to blocks in [Message](crate::message::Message) or [Attachment](crate::attachment::Attachment).
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Block {
    /// [Actions block](https://api.slack.com/reference/block-kit/blocks#actions) representation
    Actions(Box<Actions>),

    /// [Context block](https://api.slack.com/reference/block-kit/blocks#context) representation
    Context(Box<Context>),

    /// [Divider block](https://api.slack.com/reference/block-kit/blocks#divider) representation
    Divider(Box<Divider>),

    /// [Header block](https://api.slack.com/reference/block-kit/blocks#header) representation
    Header(Box<Header>),

    /// [Image block](https://api.slack.com/reference/block-kit/blocks#image) representation
    Image(Box<Image>),

    /// [Input block](https://api.slack.com/reference/block-kit/blocks#input) representation
    Input(Box<Input>),

    /// [Section block](https://api.slack.com/reference/block-kit/blocks#section) representation
    Section(Box<Section>),

    /// [Video block](https://api.slack.com/reference/block-kit/blocks#video) representation
    Video(Box<Video>),
}

impl From<Actions> for Block {
    fn from(value: Actions) -> Self {
        Self::Actions(Box::new(value))
    }
}

impl From<Context> for Block {
    fn from(value: Context) -> Self {
        Self::Context(Box::new(value))
    }
}

impl From<Divider> for Block {
    fn from(value: Divider) -> Self {
        Self::Divider(Box::new(value))
    }
}

impl From<Header> for Block {
    fn from(value: Header) -> Self {
        Self::Header(Box::new(value))
    }
}

impl From<Image> for Block {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
    }
}

impl From<Input> for Block {
    fn from(value: Input) -> Self {
        Self::Input(Box::new(value))
    }
}

impl From<Section> for Block {
    fn from(value: Section) -> Self {
        Self::Section(Box::new(value))
    }
}

impl From<Video> for Block {
    fn from(value: Video) -> Self {
        Self::Video(Box::new(value))
    }
}
