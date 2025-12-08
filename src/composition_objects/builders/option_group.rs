use super::super::types::TextInOption;
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Opt, OptGroup, PlainText};

use std::error::Error;
use std::fmt;

impl<T: TextInOption> OptGroup<T> {
    /// Construct a [`OptGroupBuilder`].
    pub fn builder() -> OptGroupBuilder<T> {
        OptGroupBuilder::default()
    }
}

/// Error while building [`OptGroup`] object.
#[derive(Debug, Clone, PartialEq, Default)]
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
    pub fn set_label(self, label: Option<PlainText>) -> Self {
        Self {
            label: new_label(label),
            ..self
        }
    }

    /// set label field value
    pub fn label(self, label: PlainText) -> Self {
        self.set_label(Some(label))
    }

    /// get options field value
    pub fn get_options(&self) -> Option<&[Opt<T>]> {
        self.options.inner_ref().map(|v| v.as_ref())
    }

    /// set options field value
    pub fn set_options(self, options: Option<Vec<Opt<T>>>) -> Self {
        Self {
            options: new_options(options),
            ..self
        }
    }

    /// set options field value
    pub fn options(self, options: Vec<Opt<T>>) -> Self {
        self.set_options(Some(options))
    }

    /// add option to options field
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
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = OptGroup {
            label: Some(plain_text("foo")),
            options: vec![option_plain("opt0", "val0"), option_plain("opt1", "val1")],
        };

        let val = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .set_options(Some(vec![
                option_plain("opt0", "val0"),
                option_plain("opt1", "val1"),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .options(vec![
                option_plain("opt0", "val0"),
                option_plain("opt1", "val1"),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_has_additional_setter_for_options_field() {
        let expected = OptGroup {
            label: Some(plain_text("foo")),
            options: vec![option_plain("opt0", "val0"), option_plain("opt1", "val1")],
        };

        let val = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .option(option_plain("opt0", "val0"))
            .option(option_plain("opt1", "val1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn label_field_is_required() {
        let err = OptGroup::builder()
            .option(option_plain("opt0", "val0"))
            .build()
            .unwrap_err();

        let expected = OptGroupError {
            label: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn label_field_length_must_be_less_than_75() {
        let err = OptGroup::builder()
            .label(plain_text("f".repeat(76)))
            .option(option_plain("opt0", "val0"))
            .build()
            .unwrap_err();

        let expected = OptGroupError {
            label: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn options_field_is_required() {
        let err = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .build()
            .unwrap_err();

        let expected = OptGroupError {
            options: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn options_item_size_must_be_less_than_100() {
        let options: Vec<Opt<PlainText>> = (0..101).map(|_| option_plain("opt", "val")).collect();

        let err = OptGroup::builder()
            .label(plain_text("foo"))
            .options(options)
            .build()
            .unwrap_err();

        let expected = OptGroupError {
            options: vec![ValidationError::MaxArraySize(100)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
