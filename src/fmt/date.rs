use chrono::prelude::*;
use regex::Regex;
use std::fmt;

const DEFAULT_FORMAT: &str = "{date} {time}";

#[derive(Debug, Clone)]
pub struct DateFormat<'a> {
    value: DateTime<FixedOffset>,
    format: &'a str,
    link: Option<&'a str>,
}

impl<'a> DateFormat<'a> {
    pub fn new(value: DateTime<FixedOffset>) -> Self {
        Self {
            value,
            format: DEFAULT_FORMAT,
            link: None,
        }
    }

    pub fn set_format(self, format: &'a str) -> Self {
        Self { format, ..self }
    }

    pub fn format(self, format: &'a str) -> Self {
        self.set_format(format)
    }

    pub fn set_link(self, link: &'a str) -> Self {
        Self {
            link: Some(link),
            ..self
        }
    }

    pub fn link(self, link: &'a str) -> Self {
        self.set_link(link)
    }

    pub fn fallback_text(&self) -> String {
        let format = FormatReplacer::new(self.format).replaced_value();
        self.value.format(&format).to_string()
    }

    fn optional_link(&self) -> String {
        if let Some(link) = self.link {
            format!("^{link}")
        } else {
            "".to_string()
        }
    }
}

impl<'a> fmt::Display for DateFormat<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<!date^{}^{}{}|{}>",
            self.value.timestamp(),
            self.format,
            self.optional_link(),
            self.fallback_text(),
        )
    }
}

impl<'a> From<DateFormat<'a>> for String {
    fn from(dt: DateFormat<'a>) -> String {
        format!("{dt}")
    }
}

#[derive(Debug)]
struct FormatReplacer {
    value: String,
}

impl FormatReplacer {
    const DATE_NUM: &str = "%Y-%m-%d";
    const DATE: &str = "%B %e, %Y";
    const DATE_SHORT: &str = "%b %e, %Y";
    const DATE_LONG: &str = "%A, %B %e, %Y";
    const TIME: &str = "%l:%M %p";
    const TIME_SECS: &str = "%l:%M:%S %p";

    fn new<T: Into<String>>(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }

    fn replace_date_num(self) -> Self {
        self.replace(r"\{date_num\}", Self::DATE_NUM)
    }

    fn replace_date(self) -> Self {
        self.replace(r"\{date\}", Self::DATE)
    }

    fn replace_date_short(self) -> Self {
        self.replace(r"\{date_short\}", Self::DATE_SHORT)
    }

    fn replace_date_long(self) -> Self {
        self.replace(r"\{date_long\}", Self::DATE_LONG)
    }

    fn replace_date_pretty(self) -> Self {
        self.replace(r"\{date_pretty\}", Self::DATE)
    }

    fn replace_date_short_pretty(self) -> Self {
        self.replace(r"\{date_short_pretty\}", Self::DATE_SHORT)
    }

    fn replace_date_long_pretty(self) -> Self {
        self.replace(r"\{date_long_pretty\}", Self::DATE_LONG)
    }

    fn replace_time(self) -> Self {
        self.replace(r"\{time\}", Self::TIME)
    }

    fn replace_time_secs(self) -> Self {
        self.replace(r"\{time_secs\}", Self::TIME_SECS)
    }

    fn replace(self, re: &str, rep: &str) -> Self {
        let re = Regex::new(re).unwrap();
        Self::new(re.replace_all(&self.value, rep))
    }

    fn replaced_value(self) -> String {
        self.replace_date_num()
            .replace_date()
            .replace_date_short()
            .replace_date_long()
            .replace_date_pretty()
            .replace_date_short_pretty()
            .replace_date_long_pretty()
            .replace_time()
            .replace_time_secs()
            .value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces_format_date_num() {
        let result = FormatReplacer::new("Posted {date_num}.")
            .replace_date_num()
            .value;
        assert_eq!(result, "Posted %Y-%m-%d.");
    }

    #[test]
    fn it_replaces_format_date() {
        let result = FormatReplacer::new("Posted {date}.").replace_date().value;
        assert_eq!(result, "Posted %B %e, %Y.");
    }

    #[test]
    fn it_replaces_format_date_short() {
        let result = FormatReplacer::new("Posted {date_short}.")
            .replace_date_short()
            .value;
        assert_eq!(result, "Posted %b %e, %Y.");
    }

    #[test]
    fn it_replaces_format_date_long() {
        let result = FormatReplacer::new("Posted {date_long}.")
            .replace_date_long()
            .value;
        assert_eq!(result, "Posted %A, %B %e, %Y.");
    }

    #[test]
    fn it_replaces_format_date_pretty() {
        let result = FormatReplacer::new("Posted {date_pretty}.")
            .replace_date_pretty()
            .value;
        assert_eq!(result, "Posted %B %e, %Y.");
    }

    #[test]
    fn it_replaces_format_date_short_pretty() {
        let result = FormatReplacer::new("Posted {date_short_pretty}.")
            .replace_date_short_pretty()
            .value;
        assert_eq!(result, "Posted %b %e, %Y.");
    }

    #[test]
    fn it_replaces_format_date_long_pretty() {
        let result = FormatReplacer::new("Posted {date_long_pretty}.")
            .replace_date_long_pretty()
            .value;
        assert_eq!(result, "Posted %A, %B %e, %Y.");
    }

    #[test]
    fn it_replaces_format_time() {
        let result = FormatReplacer::new("Posted {time}.").replace_time().value;
        assert_eq!(result, "Posted %l:%M %p.");
    }

    #[test]
    fn it_replaces_format_time_secs() {
        let result = FormatReplacer::new("Posted {time_secs}.")
            .replace_time_secs()
            .value;
        assert_eq!(result, "Posted %l:%M:%S %p.");
    }

    #[test]
    fn it_formats_with_default() {
        let f = sample();
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date} {time}|February 27, 2023 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_num() {
        let f = sample().format("{date_num} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_num} at {time}|2023-02-27 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_and_time() {
        let f = sample().format("{date} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date} at {time}|February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_short() {
        let f = sample().format("{date_short} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_short} at {time}|Feb 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_long() {
        let f = sample().format("{date_long} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_long} at {time}|Monday, February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_pretty() {
        let f = sample().format("{date_pretty} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_pretty} at {time}|February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_short_pretty() {
        let f = sample().format("{date_short_pretty} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_short_pretty} at {time}|Feb 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_long_pretty() {
        let f = sample().format("{date_long_pretty} at {time}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date_long_pretty} at {time}|Monday, February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_time_secs() {
        let f = sample().format("{date} at {time_secs}");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date} at {time_secs}|February 27, 2023 at 12:34:56 PM>"
        );
    }

    #[test]
    fn it_formats_with_optional_link() {
        let f = sample()
            .format("{date} at {time}")
            .link("https://google.com");
        assert_eq!(
            format!("{f}"),
            "<!date^1677468896^{date} at {time}^https://google.com|February 27, 2023 at 12:34 PM>"
        );
    }

    fn sample() -> DateFormat<'static> {
        // unix timestamp: 1677468896
        let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
        DateFormat::new(dt)
    }
}
