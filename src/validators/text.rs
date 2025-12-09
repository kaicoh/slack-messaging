use super::*;

use chrono::NaiveDate;
use once_cell::sync::Lazy;
use paste::paste;
use regex::Regex;
use std::error::Error;

type Text = Value<String>;

static DATE_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap());

fn max(max: usize, mut value: Text) -> Text {
    if value.inner_ref().is_some_and(|v| v.len() > max) {
        value.push(ValidationErrorKind::MaxTextLegth(max));
    }
    value
}

fn min(min: usize, mut value: Text) -> Text {
    if value.inner_ref().as_ref().is_some_and(|v| v.len() < min) {
        value.push(ValidationErrorKind::MinTextLegth(min));
    }
    value
}

fn is_valid_date(text: &str) -> Result<(), Box<dyn Error>> {
    let caps = DATE_FORMAT.captures(text).ok_or("Failed to capture text")?;
    let year: i32 = caps["year"].parse()?;
    let month: u32 = caps["month"].parse()?;
    let day: u32 = caps["day"].parse()?;
    NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?;
    Ok(())
}

macro_rules! impl_max {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_ $e>](value: Text) -> Text {
                    max($e, value)
                }
            )*
        }
    }
}

impl_max!(3000);

pub(crate) fn min_1(value: Text) -> Text {
    min(1, value)
}

pub(crate) fn date_format(mut value: Text) -> Text {
    if value.inner_ref().is_some_and(|v| is_valid_date(v).is_err()) {
        value.push(ValidationErrorKind::InvalidFormat("YYYY-MM-DD"));
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_3000_sets_error_if_the_value_has_more_than_3000_characters() {
        let text_3000 = "a".repeat(3000);
        let value: Text = Value::new(Some(text_3000));
        let result = max_3000(value);
        assert!(result.errors.is_empty());

        let text_3001 = "a".repeat(3001);
        let value: Text = Value::new(Some(text_3001));
        let result = max_3000(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::MaxTextLegth(3000)]);
    }

    #[test]
    fn min_1_sets_error_if_the_value_has_empty_string() {
        let text_1 = "a".to_string();
        let value: Text = Value::new(Some(text_1));
        let result = min_1(value);
        assert!(result.errors.is_empty());

        let text_0 = "".to_string();
        let value: Text = Value::new(Some(text_0));
        let result = min_1(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::MinTextLegth(1)]);
    }

    #[test]
    fn date_format_sets_error_if_the_value_does_not_meet_date_format() {
        let date = "2010-03-14".to_string();
        let value: Text = Value::new(Some(date));
        let result = date_format(value);
        assert!(result.errors.is_empty());

        let invalid_date = "2015-02-29".to_string();
        let value: Text = Value::new(Some(invalid_date));
        let result = date_format(value);
        assert_eq!(
            result.errors,
            vec![ValidationErrorKind::InvalidFormat("YYYY-MM-DD")]
        );

        let text = "foobar".to_string();
        let value: Text = Value::new(Some(text));
        let result = date_format(value);
        assert_eq!(
            result.errors,
            vec![ValidationErrorKind::InvalidFormat("YYYY-MM-DD")]
        );
    }
}
