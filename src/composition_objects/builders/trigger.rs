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
#[derive(Debug, Clone, PartialEq)]
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::InputParameter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let param = InputParameter::builder()
    ///     .set_name(Some("input_parameter_a"))
    ///     .value("Value for input param A")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_name(self, name: Option<impl Into<String>>) -> Self {
        Self {
            name: new_name(name.map(|v| v.into())),
            ..self
        }
    }

    /// set name field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::InputParameter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let param = InputParameter::builder()
    ///     .name("input_parameter_a")
    ///     .value("Value for input param A")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn name(self, name: impl Into<String>) -> Self {
        self.set_name(Some(name))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&JsonValue> {
        self.value.inner_ref()
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::InputParameter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let param = InputParameter::builder()
    ///     .name("input_parameter_a")
    ///     .set_value(Some("Value for input param A"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_value(self, value: Option<impl Into<JsonValue>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::InputParameter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let param = InputParameter::builder()
    ///     .name("input_parameter_a")
    ///     .value("Value for input param A")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
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
#[derive(Debug, Clone, PartialEq)]
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let trigger = Trigger::builder()
    ///     .set_url(Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz"))
    ///     .customizable_input_parameter(
    ///         InputParameter::builder()
    ///             .name("input_parameter_a")
    ///             .value("Value for input param A")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///     "customizable_input_parameters": [
    ///         {
    ///             "name": "input_parameter_a",
    ///             "value": "Value for input param A"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_url(self, url: Option<impl Into<String>>) -> Self {
        Self {
            url: new_url(url.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let trigger = Trigger::builder()
    ///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///     .customizable_input_parameter(
    ///         InputParameter::builder()
    ///             .name("input_parameter_a")
    ///             .value("Value for input param A")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///     "customizable_input_parameters": [
    ///         {
    ///             "name": "input_parameter_a",
    ///             "value": "Value for input param A"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let trigger = Trigger::builder()
    ///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///     .set_customizable_input_parameters(
    ///         Some(
    ///             vec![
    ///                 InputParameter::builder()
    ///                     .name("input_parameter_a")
    ///                     .value("Value for input param A")
    ///                     .build()?
    ///             ]
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///     "customizable_input_parameters": [
    ///         {
    ///             "name": "input_parameter_a",
    ///             "value": "Value for input param A"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
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
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let trigger = Trigger::builder()
    ///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///     .customizable_input_parameters(
    ///         vec![
    ///             InputParameter::builder()
    ///                 .name("input_parameter_a")
    ///                 .value("Value for input param A")
    ///                 .build()?
    ///         ]
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///     "customizable_input_parameters": [
    ///         {
    ///             "name": "input_parameter_a",
    ///             "value": "Value for input param A"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn customizable_input_parameters(self, params: Vec<InputParameter>) -> Self {
        self.set_customizable_input_parameters(Some(params))
    }

    /// Add parameter to customizable_input_parameters field
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let trigger = Trigger::builder()
    ///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///     .customizable_input_parameter(
    ///         InputParameter::builder()
    ///             .name("input_parameter_a")
    ///             .value("Value for input param A")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz",
    ///     "customizable_input_parameters": [
    ///         {
    ///             "name": "input_parameter_a",
    ///             "value": "Value for input param A"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
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
    fn it_builds_input_parameter() {
        let result = InputParameter::builder().name("foo").value("bar").build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = InputParameter {
            name: Some("foo".into()),
            value: Some("bar".into()),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_input_parameter_builder_returns_error() {
        let result = InputParameter::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = InputParameterError {
            name: vec![ValidationError::Required],
            value: vec![ValidationError::Required],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn it_builds_trigger() {
        let result = Trigger::builder()
            .url("foo")
            .customizable_input_parameter(
                InputParameter::builder()
                    .name("bar")
                    .value("baz")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = Trigger {
            url: Some("foo".into()),
            customizable_input_parameters: vec![InputParameter {
                name: Some("bar".into()),
                value: Some("baz".into()),
            }],
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_trigger_builder_returns_error() {
        let result = Trigger::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = TriggerError {
            url: vec![ValidationError::Required],
            customizable_input_parameters: vec![],
        };
        assert_eq!(err, expected);
    }
}
