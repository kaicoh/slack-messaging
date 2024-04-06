use serde::Serialize;
use serde_json::Value;

/// [Input parameter object](https://api.slack.com/reference/block-kit/composition-objects#input_parameter) representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::InputParameter;
/// let param = InputParameter::builder()
///     .name("input_parameter_a")
///     .value("Value for input param A")
///     .build();
///
/// let expected = serde_json::json!({
///     "name": "input_parameter_a",
///     "value": "Value for input param A"
/// });
///
/// let json = serde_json::to_value(param).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct InputParameter {
    name: String,
    value: Value,
}

impl InputParameter {
    /// Construct a [`InputParameterBuilder`].
    pub fn builder() -> InputParameterBuilder {
        InputParameterBuilder::default()
    }
}

/// Builder for [`InputParameter`] object.
#[derive(Debug, Default)]
pub struct InputParameterBuilder {
    name: Option<String>,
    value: Option<Value>,
}

impl InputParameterBuilder {
    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::InputParameter;
    /// let param = InputParameter::builder()
    ///     .set_name(Some("input_parameter_a".into()))
    ///     .value("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": ""
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_name(self, name: Option<String>) -> Self {
        Self { name, ..self }
    }

    /// Set name field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::InputParameter;
    /// let param = InputParameter::builder()
    ///     .name("input_parameter_a")
    ///     .value("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "name": "input_parameter_a",
    ///     "value": ""
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn name(self, name: impl Into<String>) -> Self {
        self.set_name(Some(name.into()))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::InputParameter;
    /// use serde_json::Value;
    ///
    /// let param = InputParameter::builder()
    ///     .name("")
    ///     .set_value(Some(Value::String("Value for input param A".into())))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "name": "",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<Value>) -> Self {
        Self { value, ..self }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::InputParameter;
    /// let param = InputParameter::builder()
    ///     .name("")
    ///     .value("Value for input param A")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "name": "",
    ///     "value": "Value for input param A"
    /// });
    ///
    /// let json = serde_json::to_value(param).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<Value>) -> Self {
        self.set_value(Some(value.into()))
    }

    /// Build a [`InputParameter`] object. This method will panic if both `name` and `value` are
    /// not set.
    pub fn build(self) -> InputParameter {
        InputParameter {
            name: self
                .name
                .expect("name must be set to InputParameterBuilder"),
            value: self
                .value
                .expect("value must be set to InputParameterBuilder"),
        }
    }
}
