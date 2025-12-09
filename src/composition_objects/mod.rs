pub mod types;

mod confirmation_dialog;
mod markdown_text;
mod plain_text;
mod text;

pub use confirmation_dialog::{ConfirmationDialog, ConfirmationDialogBuilder};
pub use markdown_text::{MrkdwnText, MrkdwnTextBuilder};
pub use plain_text::{PlainText, PlainTextBuilder};
pub use text::Text;

#[cfg(test)]
mod test_helpers {
    use super::*;

    pub fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
