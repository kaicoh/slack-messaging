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
mod icon_button;
mod image;
mod multi_select_menus;
mod number_input;
mod overflow_menu;
mod plain_text_input;
mod radio_button_group;
mod select_menus;
mod time_picker;

pub use button::Button;
pub use checkboxes::Checkboxes;
pub use date_picker::DatePicker;
pub use datetime_picker::DatetimePicker;
pub use email_input::EmailInput;
pub use feedback_buttons::FeedbackButtons;
pub use file_input::FileInput;
pub use icon_button::IconButton;
pub use image::Image;
pub use multi_select_menus::{
    MultiSelectMenuConversations, MultiSelectMenuExternalDataSource, MultiSelectMenuPublicChannels,
    MultiSelectMenuStaticOptions, MultiSelectMenuUsers,
};
pub use number_input::NumberInput;
pub use overflow_menu::OverflowMenu;
pub use plain_text_input::PlainTextInput;
pub use radio_button_group::RadioButtonGroup;
pub use select_menus::{
    SelectMenuConversations, SelectMenuExternalDataSource, SelectMenuPublicChannels,
    SelectMenuStaticOptions, SelectMenuUsers,
};
pub use time_picker::TimePicker;

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
