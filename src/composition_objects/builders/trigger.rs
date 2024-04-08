use super::{InputParameter, Trigger};

impl Trigger {
    /// Construct a [`TriggerBuilder`].
    pub fn builder() -> TriggerBuilder {
        TriggerBuilder::default()
    }
}

/// Builder for [`Trigger`] object.
#[derive(Debug, Default)]
pub struct TriggerBuilder {
    url: Option<String>,
    customizable_input_parameters: Vec<InputParameter>,
}

impl TriggerBuilder {
    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// let trigger = Trigger::builder()
    ///     .set_url(Some("https://slack.com/shortcuts/Ft0123ABC456/123...xyz".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz"
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_url(self, url: Option<String>) -> Self {
        Self { url, ..self }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// let trigger = Trigger::builder()
    ///     .url("https://slack.com/shortcuts/Ft0123ABC456/123...xyz")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://slack.com/shortcuts/Ft0123ABC456/123...xyz"
    /// });
    ///
    /// let json = serde_json::to_value(trigger).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url.into()))
    }

    /// Set customizable_input_parameters field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// let trigger = Trigger::builder()
    ///     .url("")
    ///     .set_customizable_input_parameters(
    ///         vec![
    ///             InputParameter::builder()
    ///                 .name("input_parameter_a")
    ///                 .value("Value for input param A")
    ///                 .build()
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "",
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
    /// ```
    pub fn set_customizable_input_parameters(
        self,
        customizable_input_parameters: Vec<InputParameter>,
    ) -> Self {
        Self {
            customizable_input_parameters,
            ..self
        }
    }

    /// Add input parameter object to customizable_input_parameters field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{InputParameter, Trigger};
    /// let trigger = Trigger::builder()
    ///     .url("")
    ///     .customizable_input_parameter(
    ///         InputParameter::builder()
    ///             .name("input_parameter_a")
    ///             .value("Value for input param A")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "url": "",
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
    /// ```
    pub fn customizable_input_parameter(
        self,
        customizable_input_parameter: InputParameter,
    ) -> Self {
        let Self {
            mut customizable_input_parameters,
            ..
        } = self;
        customizable_input_parameters.push(customizable_input_parameter);
        Self {
            customizable_input_parameters,
            ..self
        }
    }

    /// Build a [`Trigger`] object. This method will panic if `url` is not set.
    pub fn build(self) -> Trigger {
        Trigger {
            url: self.url.expect("url must be set to TriggerBuilder"),
            customizable_input_parameters: self.customizable_input_parameters,
        }
    }

    /// Get url value.
    pub fn get_url(&self) -> &Option<String> {
        &self.url
    }

    /// Get customizable_input_parameters value.
    pub fn get_customizable_input_parameters(&self) -> &[InputParameter] {
        &self.customizable_input_parameters
    }
}
