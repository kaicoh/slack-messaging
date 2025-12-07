use super::composition_objects::ConfirmationDialog;
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, DatetimePicker};

use std::error::Error;
use std::fmt;

impl DatetimePicker {
    /// Construct a [`DatetimePickerBuilder`].
    pub fn builder() -> DatetimePickerBuilder {
        DatetimePickerBuilder::default()
    }
}

/// Error while building [`DatetimePicker`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DatetimePickerError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of initial_date_time field
    pub initial_date_time: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of focus_on_load field
    pub focus_on_load: Vec<ValidationError>,
}

impl fmt::Display for DatetimePickerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DatetimePickerError {{ action_id: {:?}, initial_date_time: {:?}, confirm: {:?}, focus_on_load: {:?} }}",
            self.action_id, self.initial_date_time, self.confirm, self.focus_on_load
        )
    }
}

impl Error for DatetimePickerError {}

/// Builder for [`DatetimePicker`] object.
#[derive(Debug)]
pub struct DatetimePickerBuilder {
    action_id: Value<String>,
    initial_date_time: Value<i64>,
    confirm: Value<ConfirmationDialog>,
    focus_on_load: Value<bool>,
}

impl Default for DatetimePickerBuilder {
    fn default() -> Self {
        DatetimePickerBuilder {
            action_id: new_action_id(None),
            initial_date_time: new_initial_date_time(None),
            confirm: new_confirm(None),
            focus_on_load: new_focus_on_load(None),
        }
    }
}

impl Builder for DatetimePickerBuilder {
    type Target = DatetimePicker;
    type Error = DatetimePickerError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            initial_date_time,
            confirm,
            focus_on_load,
        } = self;
        value::merge_4(action_id, initial_date_time, confirm, focus_on_load)
            .map(
                |(action_id, initial_date_time, confirm, focus_on_load)| DatetimePicker {
                    action_id,
                    initial_date_time,
                    confirm,
                    focus_on_load,
                },
            )
            .map_err(
                |(action_id, initial_date_time, confirm, focus_on_load)| DatetimePickerError {
                    action_id,
                    initial_date_time,
                    confirm,
                    focus_on_load,
                },
            )
    }
}

impl DatetimePickerBuilder {
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

    /// get initial_date_time field value
    pub fn get_initial_date_time(&self) -> Option<i64> {
        self.initial_date_time.inner_ref().copied()
    }

    /// set options field value
    pub fn set_initial_date_time(self, datetime: Option<i64>) -> Self {
        Self {
            initial_date_time: new_initial_date_time(datetime),
            ..self
        }
    }

    /// set options field value
    pub fn initial_date_time(self, date: i64) -> Self {
        self.set_initial_date_time(Some(date))
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
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_initial_date_time(options: Option<i64>) -> Value<i64> {
    pipe! { Value::new(options) => validators::integer::ten_digits }
}

fn new_confirm(confirm: Option<ConfirmationDialog>) -> Value<ConfirmationDialog> {
    pipe! { Value::new(confirm) => validators::do_nothing }
}

fn new_focus_on_load(focus: Option<bool>) -> Value<bool> {
    pipe! { Value::new(focus) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = DatetimePicker {
            action_id: Some("datetimepicker-0".into()),
            initial_date_time: Some(1234567890),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
        };

        let val = DatetimePicker::builder()
            .set_action_id(Some("datetimepicker-0"))
            .set_initial_date_time(Some(1234567890))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DatetimePicker::builder()
            .action_id("datetimepicker-0")
            .initial_date_time(1234567890)
            .confirm(confirm())
            .focus_on_load(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_must_be_less_than_255() {
        let err = DatetimePicker::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();

        let expected = DatetimePickerError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn initial_date_time_should_be_date_format() {
        let err = DatetimePicker::builder()
            .initial_date_time(1000)
            .build()
            .unwrap_err();

        let expected = DatetimePickerError {
            initial_date_time: vec![ValidationError::InvalidFormat("10 digits")],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
