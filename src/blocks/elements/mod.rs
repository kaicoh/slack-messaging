/// builder objects for block elements.
pub mod builders;
/// Additional types to create block elements.
pub mod types;

mod button;
mod checkboxes;
mod date_picker;
mod datetime_picker;
mod email_input;
mod feedback_buttons;
mod file_input;

pub use button::Button;
pub use checkboxes::Checkboxes;
pub use date_picker::DatePicker;
pub use datetime_picker::DatetimePicker;
pub use email_input::EmailInput;
pub use feedback_buttons::FeedbackButtons;
pub use file_input::FileInput;

#[cfg(test)]
pub mod test_helpers {
    use super::types::*;
    use crate::composition_objects::test_helpers::*;

    pub fn fb_btn(text: impl Into<String>, value: impl Into<String>) -> FeedbackButton {
        FeedbackButton {
            text: Some(plain_text(text)),
            value: Some(value.into()),
            accessibility_label: None,
        }
    }
}
