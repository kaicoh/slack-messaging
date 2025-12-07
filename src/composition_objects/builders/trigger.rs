use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, InputParameter, Trigger};

use serde_json::Value as JsonValue;
use std::error::Error;
use std::fmt;

impl InputParameter {
    /// Construct a [`InputParameterBuilder`].
    pub fn builder() -> InputParameterBuilder {
        InputParameterBuilder::default()
    }
}

/// Error while building [`InputParameter`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct InputParameterError {
    /// errors of name field
    pub name: Vec<ValidationError>,

    /// errors of value filed
    pub value: Vec<ValidationError>,
}

impl fmt::Display for InputParameterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "InputParameterError {{ name: {:?}, value: {:?} }}",
            self.name, self.value,
        )
    }
}

impl Error for InputParameterError {}

/// Builder for [`InputParameter`] object.
#[derive(Debug)]
pub struct InputParameterBuilder {
    name: Value<String>,
    value: Value<JsonValue>,
}

impl Default for InputParameterBuilder {
    fn default() -> Self {
        InputParameterBuilder {
            name: new_name(None),
            value: new_value(None),
        }
    }
}

impl Builder for InputParameterBuilder {
    type Target = InputParameter;
    type Error = InputParameterError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        value::merge_2(self.name, self.value)
            .map(|(name, value)| InputParameter { name, value })
            .map_err(|(name, value)| InputParameterError { name, value })
    }
}

impl InputParameterBuilder {
    /// get name field value
    pub fn get_name(&self) -> Option<&String> {
        self.name.inner_ref()
    }

    /// set name field value
    pub fn set_name(self, name: Option<impl Into<String>>) -> Self {
        Self {
            name: new_name(name.map(|v| v.into())),
            ..self
        }
    }

    /// set name field value
    pub fn name(self, name: impl Into<String>) -> Self {
        self.set_name(Some(name))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&JsonValue> {
        self.value.inner_ref()
    }

    /// set value field value
    pub fn set_value(self, value: Option<impl Into<JsonValue>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    pub fn value(self, value: impl Into<JsonValue>) -> Self {
        self.set_value(Some(value))
    }
}

fn new_name(name: Option<String>) -> Value<String> {
    pipe! { Value::new(name) => validators::required }
}

fn new_value(value: Option<JsonValue>) -> Value<JsonValue> {
    pipe! { Value::new(value) => validators::required }
}

impl Trigger {
    /// Construct a [`TriggerBuilder`].
    pub fn builder() -> TriggerBuilder {
        TriggerBuilder::default()
    }
}

/// Error while building [`Trigger`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TriggerError {
    /// errors of url field
    pub url: Vec<ValidationError>,

    /// errors of customizable_input_parameters filed
    pub customizable_input_parameters: Vec<ValidationError>,
}

impl fmt::Display for TriggerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TriggerError {{ url: {:?}, customizable_input_parameters: {:?} }}",
            self.url, self.customizable_input_parameters,
        )
    }
}

impl Error for TriggerError {}

/// Builder for [`Trigger`] object.
#[derive(Debug)]
pub struct TriggerBuilder {
    url: Value<String>,
    customizable_input_parameters: Value<Vec<InputParameter>>,
}

impl Default for TriggerBuilder {
    fn default() -> Self {
        TriggerBuilder {
            url: new_url(None),
            customizable_input_parameters: new_customizable_input_parameters(None),
        }
    }
}

impl Builder for TriggerBuilder {
    type Target = Trigger;
    type Error = TriggerError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        value::merge_2(self.url, self.customizable_input_parameters)
            .map(|(url, customizable_input_parameters)| Trigger {
                url,
                customizable_input_parameters: customizable_input_parameters.unwrap_or_default(),
            })
            .map_err(|(url, customizable_input_parameters)| TriggerError {
                url,
                customizable_input_parameters,
            })
    }
}

impl TriggerBuilder {
    /// get url field value
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value
    pub fn set_url(self, url: Option<impl Into<String>>) -> Self {
        Self {
            url: new_url(url.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url))
    }

    /// get customizable_input_parameters field value
    pub fn get_customizable_input_parameters(&self) -> Option<&[InputParameter]> {
        self.customizable_input_parameters
            .inner_ref()
            .map(|v| v.as_ref())
    }

    /// set customizable_input_parameters field value
    pub fn set_customizable_input_parameters(
        self,
        customizable_input_parameters: Option<Vec<InputParameter>>,
    ) -> Self {
        Self {
            customizable_input_parameters: new_customizable_input_parameters(
                customizable_input_parameters,
            ),
            ..self
        }
    }

    /// set customizable_input_parameters field value
    pub fn customizable_input_parameters(self, params: Vec<InputParameter>) -> Self {
        self.set_customizable_input_parameters(Some(params))
    }

    /// Add parameter to customizable_input_parameters field
    pub fn customizable_input_parameter(mut self, param: InputParameter) -> Self {
        let mut list = self
            .customizable_input_parameters
            .take_inner()
            .unwrap_or_default();
        list.push(param);
        self.customizable_input_parameters(list)
    }
}

fn new_url(url: Option<String>) -> Value<String> {
    pipe! { Value::new(url) => validators::required }
}

fn new_customizable_input_parameters(
    customizable_input_parameters: Option<Vec<InputParameter>>,
) -> Value<Vec<InputParameter>> {
    pipe! { Value::new(customizable_input_parameters) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_parameter_builder_has_setter_methods() {
        let expected = InputParameter {
            name: Some("foo".into()),
            value: Some("bar".into()),
        };

        let val = InputParameter::builder()
            .set_name(Some("foo"))
            .set_value(Some("bar"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = InputParameter::builder()
            .name("foo")
            .value("bar")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn name_field_of_input_parameter_builder_is_required() {
        let err = InputParameter::builder().value("bar").build().unwrap_err();

        let expected = InputParameterError {
            name: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_of_input_parameter_builder_is_required() {
        let err = InputParameter::builder().name("foo").build().unwrap_err();

        let expected = InputParameterError {
            value: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn trigger_builder_has_setter_methods() {
        let expected = Trigger {
            url: Some("foobarbaz".into()),
            customizable_input_parameters: vec![InputParameter {
                name: Some("foo".into()),
                value: Some("bar".into()),
            }],
        };

        let val = Trigger::builder()
            .set_url(Some("foobarbaz"))
            .set_customizable_input_parameters(Some(vec![InputParameter {
                name: Some("foo".into()),
                value: Some("bar".into()),
            }]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Trigger::builder()
            .url("foobarbaz")
            .customizable_input_parameters(vec![InputParameter {
                name: Some("foo".into()),
                value: Some("bar".into()),
            }])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn trigger_builder_has_additional_setter_for_customizable_input_parameters_field() {
        let expected = Trigger {
            url: Some("foobarbaz".into()),
            customizable_input_parameters: vec![
                InputParameter {
                    name: Some("foo".into()),
                    value: Some("bar".into()),
                },
                InputParameter {
                    name: Some("baz".into()),
                    value: Some("foo".into()),
                },
            ],
        };

        let val = Trigger::builder()
            .url("foobarbaz")
            .customizable_input_parameter(InputParameter {
                name: Some("foo".into()),
                value: Some("bar".into()),
            })
            .customizable_input_parameter(InputParameter {
                name: Some("baz".into()),
                value: Some("foo".into()),
            })
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn url_field_of_trigger_builder_is_required() {
        let err = Trigger::builder()
            .customizable_input_parameter(InputParameter {
                name: Some("foo".into()),
                value: Some("bar".into()),
            })
            .build()
            .unwrap_err();

        let expected = TriggerError {
            url: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
