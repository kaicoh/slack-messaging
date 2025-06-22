use super::{RichText, composition_objects};

/// Builder objects for Block elements.
pub mod builders;

mod button;
mod checkboxes;
mod datepicker;
mod datetimepicker;
mod email_input;
mod file_input;
mod image;
mod multi_select_menu;
mod number_input;
mod overflow_menu;
mod plain_text_input;
mod radio_button_group;
mod rich_text_input;
mod select_menu;
mod timepicker;
mod url_input;
mod workflow_button;

pub use button::Button;
pub use checkboxes::Checkboxes;
pub use datepicker::DatePicker;
pub use datetimepicker::DatetimePicker;
pub use email_input::EmailInput;
pub use file_input::{FileInput, FileType};
pub use image::Image;
pub use multi_select_menu::{
    MultiSelect, MultiSelectConversations, MultiSelectExternals, MultiSelectMenu,
    MultiSelectPublicChannels, MultiSelectStaticOptions, MultiSelectUsers,
};
pub use number_input::NumberInput;
pub use overflow_menu::OverflowMenu;
pub use plain_text_input::PlainTextInput;
pub use radio_button_group::RadioButtonGroup;
pub use rich_text_input::RichTextInput;
pub use select_menu::{
    Select, SelectConversations, SelectExternals, SelectMenu, SelectPublicChannels,
    SelectStaticOptions, SelectUsers,
};
pub use timepicker::TimePicker;
pub use url_input::UrlInput;
pub use workflow_button::WorkflowButton;

#[derive(Debug, Default, Copy, Clone)]
pub struct StaticOptions;

#[derive(Debug, Default, Copy, Clone)]
pub struct ExternalDataSource;

#[derive(Debug, Default, Copy, Clone)]
pub struct UserList;

#[derive(Debug, Default, Copy, Clone)]
pub struct ConversationsList;

#[derive(Debug, Default, Copy, Clone)]
pub struct PublicChannels;
