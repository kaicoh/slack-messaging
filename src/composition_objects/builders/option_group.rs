use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Opt, OptGroup, PlainText, TextInOption};

use std::error::Error;
use std::fmt;

impl<T: TextInOption> OptGroup<T> {
    /// Construct a [`OptGroupBuilder`].
    pub fn builder() -> OptGroupBuilder<T> {
        OptGroupBuilder::default()
    }
}

/// Error while building [`OptGroup`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct OptGroupError {
    /// errors of label field
    pub label: Vec<ValidationError>,

    /// errors of options field
    pub options: Vec<ValidationError>,
}

impl fmt::Display for OptGroupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OptGroupError {{ label: {:?}, options: {:?} }}",
            self.label, self.options
        )
    }
}

impl Error for OptGroupError {}

/// Builder for [`OptGroup`] object.
#[derive(Debug)]
pub struct OptGroupBuilder<T: TextInOption> {
    label: Value<PlainText>,
    options: Value<Vec<Opt<T>>>,
}

impl<T: TextInOption> Default for OptGroupBuilder<T> {
    fn default() -> Self {
        OptGroupBuilder {
            label: new_label(None),
            options: new_options(None),
        }
    }
}

impl<T: TextInOption> Builder for OptGroupBuilder<T> {
    type Target = OptGroup<T>;
    type Error = OptGroupError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self { label, options } = self;

        value::merge_2(label, options)
            .map(|(label, options)| OptGroup {
                label,
                options: options.unwrap_or_default(),
            })
            .map_err(|(label, options)| OptGroupError { label, options })
    }
}

impl<T: TextInOption> OptGroupBuilder<T> {
    /// get label field value
    pub fn get_label(&self) -> Option<&PlainText> {
        self.label.inner_ref()
    }

    /// set label field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let options = OptGroup::<PlainText>::builder()
    ///     .set_label(Some(plain_text!("Group One")?))
    ///     .option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0")?)
    ///             .value("value-0")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_label(self, label: Option<PlainText>) -> Self {
        Self {
            label: new_label(label),
            ..self
        }
    }

    /// set label field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let options = OptGroup::<PlainText>::builder()
    ///     .label(plain_text!("Group One")?)
    ///     .option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0")?)
    ///             .value("value-0")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn label(self, label: PlainText) -> Self {
        self.set_label(Some(label))
    }

    /// get options field value
    pub fn get_options(&self) -> Option<&[Opt<T>]> {
        self.options.inner_ref().map(|v| v.as_ref())
    }

    /// set options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let options = OptGroup::<PlainText>::builder()
    ///     .label(plain_text!("Group One")?)
    ///     .set_options(
    ///         Some(
    ///             vec![
    ///                 Opt::builder()
    ///                     .text(plain_text!("option-0")?)
    ///                     .value("value-0")
    ///                     .build()?
    ///             ]
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_options(self, options: Option<Vec<Opt<T>>>) -> Self {
        Self {
            options: new_options(options),
            ..self
        }
    }

    /// set options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let options = OptGroup::<PlainText>::builder()
    ///     .label(plain_text!("Group One")?)
    ///     .options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(plain_text!("option-0")?)
    ///                 .value("value-0")
    ///                 .build()?
    ///         ]
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn options(self, options: Vec<Opt<T>>) -> Self {
        self.set_options(Some(options))
    }

    /// add option to options field
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let options = OptGroup::<PlainText>::builder()
    ///     .label(plain_text!("Group One")?)
    ///     .option(
    ///         Opt::builder()
    ///             .text(plain_text!("option-0")?)
    ///             .value("value-0")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn option(mut self, option: Opt<T>) -> Self {
        let mut list = self.options.take_inner().unwrap_or_default();
        list.push(option);
        self.options(list)
    }
}

fn new_label(label: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(label) =>
            validators::required |
            validators::text_object::max_75
    }
}

fn new_options<T: TextInOption>(options: Option<Vec<Opt<T>>>) -> Value<Vec<Opt<T>>> {
    pipe! {
        Value::new(options) =>
            validators::required |
            validators::list::max_item_100
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_option_group() {
        let result = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .option(
                Opt::builder()
                    .text(plain_text("bar"))
                    .value("baz")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = OptGroup {
            label: Some(plain_text("foo")),
            options: vec![option("bar", "baz")],
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_builder_returns_error() {
        let result = OptGroup::<PlainText>::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptGroupError {
            label: vec![ValidationError::Required],
            options: vec![ValidationError::Required],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_75_error_if_label_length_is_more_than_75() {
        let result = OptGroup::<PlainText>::builder()
            .label(plain_text("f".repeat(76)))
            .option(
                Opt::builder()
                    .text(plain_text("bar"))
                    .value("baz")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptGroupError {
            label: vec![ValidationError::MaxTextLegth(75)],
            options: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_item_100_error_if_options_item_size_is_more_than_100() {
        let options: Vec<Opt<PlainText>> = (0..101)
            .map(|_| {
                Opt::builder()
                    .text(plain_text("bar"))
                    .value("baz")
                    .build()
                    .unwrap()
            })
            .collect();

        let result = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .options(options)
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = OptGroupError {
            label: vec![],
            options: vec![ValidationError::MaxArraySize(100)],
        };
        assert_eq!(err, expected);
    }

    fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }

    fn option(text: impl Into<String>, value: impl Into<String>) -> Opt<PlainText> {
        Opt {
            text: Some(plain_text(text)),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }
}
