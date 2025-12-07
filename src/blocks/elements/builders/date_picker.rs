use super::composition_objects::{ConfirmationDialog, PlainText};
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, DatePicker};

use std::error::Error;
use std::fmt;

impl DatePicker {
    /// Construct a [`DatePickerBuilder`].
    pub fn builder() -> DatePickerBuilder {
        DatePickerBuilder::default()
    }
}

/// Error while building [`DatePicker`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DatePickerError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of initial_date field
    pub initial_date: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of focus_on_load field
    pub focus_on_load: Vec<ValidationError>,

    /// errors of placeholder field
    pub placeholder: Vec<ValidationError>,
}

impl fmt::Display for DatePickerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DatePickerError {{ action_id: {:?}, initial_date: {:?}, confirm: {:?}, focus_on_load: {:?}, placeholder: {:?} }}",
            self.action_id, self.initial_date, self.confirm, self.focus_on_load, self.placeholder
        )
    }
}

impl Error for DatePickerError {}

/// Builder for [`DatePicker`] object.
#[derive(Debug)]
pub struct DatePickerBuilder {
    action_id: Value<String>,
    initial_date: Value<String>,
    confirm: Value<ConfirmationDialog>,
    focus_on_load: Value<bool>,
    placeholder: Value<PlainText>,
}

impl Default for DatePickerBuilder {
    fn default() -> Self {
        DatePickerBuilder {
            action_id: new_action_id(None),
            initial_date: new_initial_date(None),
            confirm: new_confirm(None),
            focus_on_load: new_focus_on_load(None),
            placeholder: new_placeholder(None),
        }
    }
}

impl Builder for DatePickerBuilder {
    type Target = DatePicker;
    type Error = DatePickerError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            initial_date,
            confirm,
            focus_on_load,
            placeholder,
        } = self;
        value::merge_5(action_id, initial_date, confirm, focus_on_load, placeholder)
            .map(
                |(action_id, initial_date, confirm, focus_on_load, placeholder)| DatePicker {
                    action_id,
                    initial_date,
                    confirm,
                    focus_on_load,
                    placeholder,
                },
            )
            .map_err(
                |(action_id, initial_date, confirm, focus_on_load, placeholder)| DatePickerError {
                    action_id,
                    initial_date,
                    confirm,
                    focus_on_load,
                    placeholder,
                },
            )
    }
}

impl DatePickerBuilder {
    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    pub fn set_action_id(self, action_id: Option<impl Into<String>>) -> Self {
        Self {
            action_id: new_action_id(action_id.map(|v| v.into())),
            ..self
        }
    }

    /// set action_id field value
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id))
    }

    /// get initial_date field value
    pub fn get_initial_date(&self) -> Option<&String> {
        self.initial_date.inner_ref()
    }

    /// set options field value
    pub fn set_initial_date(self, date: Option<impl Into<String>>) -> Self {
        Self {
            initial_date: new_initial_date(date.map(|v| v.into())),
            ..self
        }
    }

    /// set options field value
    pub fn initial_date(self, date: impl Into<String>) -> Self {
        self.set_initial_date(Some(date))
    }

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&ConfirmationDialog> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self {
            confirm: new_confirm(confirm),
            ..self
        }
    }

    /// set confirm field value
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// get focus_on_load field value
    pub fn get_focus_on_load(&self) -> Option<bool> {
        self.focus_on_load.inner_ref().copied()
    }

    /// set focus_on_load field value
    pub fn set_focus_on_load(self, focus_on_load: Option<bool>) -> Self {
        Self {
            focus_on_load: new_focus_on_load(focus_on_load),
            ..self
        }
    }

    /// set focus_on_load field value
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }

    /// get placeholder field value
    pub fn get_placeholder(&self) -> Option<&PlainText> {
        self.placeholder.inner_ref()
    }

    /// set placeholder field value
    pub fn set_placeholder(self, placeholder: Option<PlainText>) -> Self {
        Self {
            placeholder: new_placeholder(placeholder),
            ..self
        }
    }

    /// set placeholder field value
    pub fn placeholder(self, placeholder: PlainText) -> Self {
        self.set_placeholder(Some(placeholder))
    }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_initial_date(options: Option<String>) -> Value<String> {
    pipe! { Value::new(options) => validators::text::date_format }
}

fn new_confirm(confirm: Option<ConfirmationDialog>) -> Value<ConfirmationDialog> {
    pipe! { Value::new(confirm) => validators::do_nothing }
}

fn new_focus_on_load(focus: Option<bool>) -> Value<bool> {
    pipe! { Value::new(focus) => validators::do_nothing }
}

fn new_placeholder(placeholder: Option<PlainText>) -> Value<PlainText> {
    pipe! { Value::new(placeholder) => validators::text_object::max_150 }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = DatePicker {
            action_id: Some("datepicker-0".into()),
            initial_date: Some("2025-12-07".into()),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("foo")),
        };

        let val = DatePicker::builder()
            .set_action_id(Some("datepicker-0"))
            .set_initial_date(Some("2025-12-07"))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("foo")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DatePicker::builder()
            .action_id("datepicker-0")
            .initial_date("2025-12-07")
            .confirm(confirm())
            .focus_on_load(true)
            .placeholder(plain_text("foo"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_must_be_less_than_255() {
        let err = DatePicker::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();

        let expected = DatePickerError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn initial_date_should_be_date_format() {
        let err = DatePicker::builder()
            .initial_date("foobar")
            .build()
            .unwrap_err();

        let expected = DatePickerError {
            initial_date: vec![ValidationError::InvalidFormat("YYYY-MM-DD")],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn placeholder_field_length_must_be_less_than_150() {
        let err = DatePicker::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();

        let expected = DatePickerError {
            placeholder: vec![ValidationError::MaxTextLegth(150)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
