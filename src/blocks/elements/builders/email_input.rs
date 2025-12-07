use super::composition_objects::{DispatchActionConfiguration, PlainText};
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, EmailInput};

use std::error::Error;
use std::fmt;

impl EmailInput {
    /// Construct a [`EmailInputBuilder`].
    pub fn builder() -> EmailInputBuilder {
        EmailInputBuilder::default()
    }
}

/// Error while building [`EmailInput`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmailInputError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of initial_value field
    pub initial_value: Vec<ValidationError>,

    /// errors of dispatch_action_config field
    pub dispatch_action_config: Vec<ValidationError>,

    /// errors of focus_on_load field
    pub focus_on_load: Vec<ValidationError>,

    /// errors of placeholder field
    pub placeholder: Vec<ValidationError>,
}

impl fmt::Display for EmailInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EmailInputError {{ action_id: {:?}, initial_value: {:?}, dispatch_action_config: {:?}, focus_on_load: {:?}, placeholder: {:?} }}",
            self.action_id,
            self.initial_value,
            self.dispatch_action_config,
            self.focus_on_load,
            self.placeholder
        )
    }
}

impl Error for EmailInputError {}

/// Builder for [`EmailInput`] object.
#[derive(Debug)]
pub struct EmailInputBuilder {
    action_id: Value<String>,
    initial_value: Value<String>,
    dispatch_action_config: Value<DispatchActionConfiguration>,
    focus_on_load: Value<bool>,
    placeholder: Value<PlainText>,
}

impl Default for EmailInputBuilder {
    fn default() -> Self {
        EmailInputBuilder {
            action_id: new_action_id(None),
            initial_value: new_initial_value(None),
            dispatch_action_config: new_dispatch_action_config(None),
            focus_on_load: new_focus_on_load(None),
            placeholder: new_placeholder(None),
        }
    }
}

impl Builder for EmailInputBuilder {
    type Target = EmailInput;
    type Error = EmailInputError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
        } = self;
        value::merge_5(
            action_id,
            initial_value,
            dispatch_action_config,
            focus_on_load,
            placeholder,
        )
        .map(
            |(action_id, initial_value, dispatch_action_config, focus_on_load, placeholder)| {
                EmailInput {
                    action_id,
                    initial_value,
                    dispatch_action_config,
                    focus_on_load,
                    placeholder,
                }
            },
        )
        .map_err(
            |(action_id, initial_value, dispatch_action_config, focus_on_load, placeholder)| {
                EmailInputError {
                    action_id,
                    initial_value,
                    dispatch_action_config,
                    focus_on_load,
                    placeholder,
                }
            },
        )
    }
}

impl EmailInputBuilder {
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

    /// get initial_value field value
    pub fn get_initial_value(&self) -> Option<&String> {
        self.initial_value.inner_ref()
    }

    /// set options field value
    pub fn set_initial_value(self, date: Option<impl Into<String>>) -> Self {
        Self {
            initial_value: new_initial_value(date.map(|v| v.into())),
            ..self
        }
    }

    /// set options field value
    pub fn initial_value(self, date: impl Into<String>) -> Self {
        self.set_initial_value(Some(date))
    }

    /// get dispatch_action_config field value
    pub fn get_dispatch_action_config(&self) -> Option<&DispatchActionConfiguration> {
        self.dispatch_action_config.inner_ref()
    }

    /// set dispatch_action_config field value
    pub fn set_dispatch_action_config(self, config: Option<DispatchActionConfiguration>) -> Self {
        Self {
            dispatch_action_config: new_dispatch_action_config(config),
            ..self
        }
    }

    /// set dispatch_action_config field value
    pub fn dispatch_action_config(self, config: DispatchActionConfiguration) -> Self {
        self.set_dispatch_action_config(Some(config))
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

fn new_initial_value(options: Option<String>) -> Value<String> {
    pipe! { Value::new(options) => validators::do_nothing }
}

fn new_dispatch_action_config(
    dispatch_action_config: Option<DispatchActionConfiguration>,
) -> Value<DispatchActionConfiguration> {
    pipe! { Value::new(dispatch_action_config) => validators::do_nothing }
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
        let expected = EmailInput {
            action_id: Some("email-input-0".into()),
            initial_value: Some("foobar@gmail.com".into()),
            dispatch_action_config: Some(dispatch_action_config()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("foo")),
        };

        let val = EmailInput::builder()
            .set_action_id(Some("email-input-0"))
            .set_initial_value(Some("foobar@gmail.com"))
            .set_dispatch_action_config(Some(dispatch_action_config()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("foo")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = EmailInput::builder()
            .action_id("email-input-0")
            .initial_value("foobar@gmail.com")
            .dispatch_action_config(dispatch_action_config())
            .focus_on_load(true)
            .placeholder(plain_text("foo"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_must_be_less_than_255() {
        let err = EmailInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();

        let expected = EmailInputError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn placeholder_field_length_must_be_less_than_150() {
        let err = EmailInput::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();

        let expected = EmailInputError {
            placeholder: vec![ValidationError::MaxTextLegth(150)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
