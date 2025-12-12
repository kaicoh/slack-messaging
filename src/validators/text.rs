use super::*;

use chrono::{NaiveDate, NaiveTime};
use once_cell::sync::Lazy;
use paste::paste;
use regex::Regex;
use std::error::Error;

type Text = Value<String>;

static DATE_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})$").unwrap());

static TIME_FORMAT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?x)(?P<hour>\d{2}):(?P<minute>\d{2})$").unwrap());

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

fn is_valid_time(text: &str) -> Result<(), Box<dyn Error>> {
    let caps = TIME_FORMAT.captures(text).ok_or("Failed to capture text")?;
    let hour: u32 = caps["hour"].parse()?;
    let minute: u32 = caps["minute"].parse()?;
    NaiveTime::from_hms_opt(hour, minute, 0).ok_or("Invalid time")?;
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

impl_max!(75, 150, 255, 2000, 3000, 12000);

pub(crate) fn min_1(value: Text) -> Text {
    min(1, value)
}

pub(crate) fn date_format(mut value: Text) -> Text {
    if value.inner_ref().is_some_and(|v| is_valid_date(v).is_err()) {
        value.push(ValidationErrorKind::InvalidFormat("YYYY-MM-DD"));
    }
    value
}

pub(crate) fn time_format(mut value: Text) -> Text {
    if value.inner_ref().is_some_and(|v| is_valid_time(v).is_err()) {
        value.push(ValidationErrorKind::InvalidFormat("24-hour format HH:mm"));
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_max_3000 {
        use super::*;

        #[test]
        fn it_sets_an_error_if_the_value_has_more_than_3000_characters() {
            let text = "a".repeat(3001);

            let result = test(text);
            assert_eq!(result.errors, vec![ValidationErrorKind::MaxTextLegth(3000)]);
        }

        #[test]
        fn it_passes_if_the_value_has_less_than_3000_characters() {
            let text = "a".repeat(3000);

            let result = test(text);
            assert!(result.errors.is_empty());
        }

        fn test(text: impl Into<String>) -> Text {
            max_3000(Value::new(Some(text.into())))
        }
    }

    mod fn_min_1 {
        use super::*;

        #[test]
        fn it_sets_an_error_if_the_value_is_empty_string() {
            let text = "".to_string();

            let result = test(text);
            assert_eq!(result.errors, vec![ValidationErrorKind::MinTextLegth(1)]);
        }

        #[test]
        fn it_passes_if_the_value_has_more_than_1_characters() {
            let text = "a".to_string();

            let result = test(text);
            assert!(result.errors.is_empty());
        }

        fn test(text: impl Into<String>) -> Text {
            min_1(Value::new(Some(text.into())))
        }
    }

    mod fn_date_format {
        use super::*;

        #[test]
        fn it_passes_if_the_value_match_the_date_format() {
            let text = "2010-03-14";

            let result = test(text);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_value_is_invalid_date() {
            let text = "2015-02-29";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("YYYY-MM-DD")]
            );
        }

        #[test]
        fn it_set_an_error_if_the_value_does_not_match_the_date_format() {
            let text = "foobar";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("YYYY-MM-DD")]
            );

            let text = "foo2025-12-11bar";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("YYYY-MM-DD")]
            );
        }

        fn test(text: impl Into<String>) -> Text {
            date_format(Value::new(Some(text.into())))
        }
    }

    mod fn_time_format {
        use super::*;

        #[test]
        fn it_passes_if_the_value_match_the_time_format() {
            let text = "00:00";

            let result = test(text);
            assert!(result.errors.is_empty());

            let text = "23:59";

            let result = test(text);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_value_is_invalid_time() {
            let text = "24:00";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("24-hour format HH:mm")]
            );

            let text = "23:60";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("24-hour format HH:mm")]
            );

            let text = "0:0";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("24-hour format HH:mm")]
            );
        }

        #[test]
        fn it_set_an_error_if_the_value_does_not_match_the_time_format() {
            let text = "foobar";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("24-hour format HH:mm")]
            );

            let text = "foo12:30bar";

            let result = test(text);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("24-hour format HH:mm")]
            );
        }

        fn test(text: impl Into<String>) -> Text {
            time_format(Value::new(Some(text.into())))
        }
    }
}
