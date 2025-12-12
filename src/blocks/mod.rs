/// Builders for blocks.
pub mod builders;
/// Objects from which blocks are composed.
pub mod elements;
/// Module for building [`RichText`] block.
pub mod rich_text;

mod actions;
mod context;
mod context_actions;
mod divider;
mod file;
mod header;
mod image;
mod input;
mod markdown;

pub use actions::{Actions, ActionsElement};
pub use context::{Context, ContextElement};
pub use context_actions::{ContextActions, ContextActionsElement};
pub use divider::Divider;
pub use file::{File, FileSource};
pub use header::Header;
pub use image::Image;
pub use input::{Input, InputElement};
pub use markdown::Markdown;
pub use rich_text::RichText;

#[cfg(test)]
pub mod test_helpers {
    use super::rich_text::test_helpers::*;
    use super::rich_text::types::test_helpers::*;
    use super::*;

    pub fn rich_text() -> RichText {
        RichText {
            block_id: Some("rich_text_0".into()),
            elements: Some(vec![section(vec![el_text("foo")]).into()]),
        }
    }
}
