use chrono::{DateTime, FixedOffset};
use once_cell::sync::Lazy;
use regex::Regex;

const DEFAULT_TOKEN: &str = "{date} {time}";

const F_DATE_NUM: (&str, &str) = ("%Y-%m-%d", r"\{date_num\}");
const F_DATE: (&str, &str) = ("%B %e, %Y", r"\{date\}");
const F_DATE_SHORT: (&str, &str) = ("%b %e, %Y", r"\{date_short\}");
const F_DATE_LONG: (&str, &str) = ("%A, %B %e, %Y", r"\{date_long\}");
const F_DATE_PRETTY: (&str, &str) = ("%B %e, %Y", r"\{date_pretty\}");
const F_DATE_SHORT_PRETTY: (&str, &str) = ("%b %e, %Y", r"\{date_short_pretty\}");
const F_DATE_LONG_PRETTY: (&str, &str) = ("%A, %B %e, %Y", r"\{date_long_pretty\}");
const F_TIME: (&str, &str) = ("%l:%M %p", r"\{time\}");
const F_TIME_SECS: (&str, &str) = ("%l:%M:%S %p", r"\{time_secs\}");

static DATE_FORMATS: Lazy<Vec<DateFormat>> = Lazy::new(|| {
    let formats = [
        F_DATE_NUM,
        F_DATE,
        F_DATE_SHORT,
        F_DATE_LONG,
        F_DATE_PRETTY,
        F_DATE_SHORT_PRETTY,
        F_DATE_LONG_PRETTY,
        F_TIME,
        F_TIME_SECS,
    ];
    formats.into_iter().map(DateFormat::new).collect()
});

#[derive(Debug)]
struct DateFormat {
    strf: &'static str,
    reg: Regex,
}

impl DateFormat {
    fn new((strf, re): (&'static str, &str)) -> Self {
        Self {
            strf,
            reg: Regex::new(re).unwrap(),
        }
    }

    fn replace_all(&self, haystack: &str) -> String {
        self.reg.replace_all(haystack, self.strf).into_owned()
    }
}

fn parse_to_strf(text: &str) -> String {
    DATE_FORMATS
        .iter()
        .fold(text.to_string(), |acc, r| r.replace_all(&acc))
}

/// A Formatter from chrono's [DateTime](https://docs.rs/chrono/0.4.23/chrono/struct.DateTime.html) to [slack's Date Format](https://api.slack.com/reference/surfaces/formatting#date-formatting).
///
/// # Example
///
/// ```
/// use chrono::prelude::*;
/// use slack_messaging::fmt::DateFormatter;
///
/// // Formatter without optional link.
/// let f = DateFormatter::builder()
///     .token("{date_short} at {time}")
///     .build();
///
/// let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
///
/// assert_eq!(
///     f.format(&dt),
///     "<!date^1677468896^{date_short} at {time}|Feb 27, 2023 at 12:34 PM>"
/// );
///
/// // You can also set optional link when formatting.
/// assert_eq!(
///     f.format_with_link(&dt, "https://example.com"),
///     "<!date^1677468896^{date_short} at {time}^https://example.com|Feb 27, 2023 at 12:34 PM>"
/// );
///
/// // Formatter with optional link.
/// let f = DateFormatter::builder()
///     .token("{date_short} at {time}")
///     .link("https://example.com")
///     .build();
///
/// // This time, format method returns text with link set to the formatter.
/// assert_eq!(
///     f.format(&dt),
///     "<!date^1677468896^{date_short} at {time}^https://example.com|Feb 27, 2023 at 12:34 PM>"
/// );
/// ```
#[derive(Debug, Clone)]
pub struct DateFormatter {
    token: String,
    link: Option<String>,
}

impl Default for DateFormatter {
    /// This returns DateFormatter with token string `{date} {time}` and without link.
    ///
    /// ```
    /// # use chrono::prelude::*;
    /// # use slack_messaging::fmt::DateFormatter;
    /// let f = DateFormatter::default();
    /// let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
    ///
    /// assert_eq!(
    ///     f.format(&dt),
    ///     "<!date^1677468896^{date} {time}|February 27, 2023 12:34 PM>",
    /// );
    /// ```
    fn default() -> Self {
        Self {
            token: DEFAULT_TOKEN.into(),
            link: None,
        }
    }
}

impl DateFormatter {
    /// Construct a [`DateFormatterBuilder`]
    pub fn builder() -> DateFormatterBuilder {
        DateFormatterBuilder::default()
    }

    /// Return formatted string.
    ///
    /// ```
    /// # use chrono::prelude::*;
    /// # use slack_messaging::fmt::DateFormatter;
    /// // If formatter doesn't have a link, this method returns string without optional link.
    /// let f = DateFormatter::builder()
    ///     .token("{date_pretty} at {time}")
    ///     .build();
    /// let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
    ///
    /// assert_eq!(
    ///     f.format(&dt),
    ///     "<!date^1677468896^{date_pretty} at {time}|February 27, 2023 at 12:34 PM>",
    /// );
    ///
    /// // If formatter has a link, this method returns string with it.
    /// let f = DateFormatter::builder()
    ///     .token("{date} at {time}")
    ///     .link("https://google.com")
    ///     .build();
    ///
    /// assert_eq!(
    ///     f.format(&dt),
    ///     "<!date^1677468896^{date} at {time}^https://google.com|February 27, 2023 at 12:34 PM>",
    /// );
    /// ```
    pub fn format(&self, dt: &DateTime<FixedOffset>) -> String {
        self.fmt(dt, self.link.as_deref())
    }

    /// Return formatted string with optional link.
    ///
    /// ```
    /// # use chrono::prelude::*;
    /// # use slack_messaging::fmt::DateFormatter;
    /// // If formatter doesn't have a link, this method returns string with link given as an argument.
    /// let f = DateFormatter::builder()
    ///     .token("{date_pretty} at {time}")
    ///     .build();
    /// let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
    ///
    /// assert_eq!(
    ///     f.format_with_link(&dt, "https://amazon.com"),
    ///     "<!date^1677468896^{date_pretty} at {time}^https://amazon.com|February 27, 2023 at 12:34 PM>",
    /// );
    ///
    /// // Even if formatter has a link, this method also returns string with link given as an argument.
    /// let f = DateFormatter::builder()
    ///     .token("{date} at {time}")
    ///     .link("https://google.com")
    ///     .build();
    ///
    /// assert_eq!(
    ///     f.format_with_link(&dt, "https://amazon.com"),
    ///     "<!date^1677468896^{date} at {time}^https://amazon.com|February 27, 2023 at 12:34 PM>",
    /// );
    /// ```
    pub fn format_with_link(&self, dt: &DateTime<FixedOffset>, link: &str) -> String {
        self.fmt(dt, Some(link))
    }

    fn fmt(&self, dt: &DateTime<FixedOffset>, link: Option<&str>) -> String {
        let link = match link {
            Some(link) => format!("^{link}"),
            None => "".into(),
        };

        format!(
            "<!date^{}^{}{}|{}>",
            dt.timestamp(),
            self.token,
            link,
            dt.format(&parse_to_strf(&self.token)),
        )
    }
}

/// Builder for [`DateFormatter`] object.
#[derive(Debug, Default)]
pub struct DateFormatterBuilder {
    token: Option<String>,
    link: Option<String>,
}

impl DateFormatterBuilder {
    /// Set token string to format timestamp.
    pub fn set_token(self, token: Option<String>) -> Self {
        Self { token, ..self }
    }

    /// Set token string to format timestamp.
    pub fn token(self, token: impl Into<String>) -> Self {
        self.set_token(Some(token.into()))
    }

    /// Set optional link.
    pub fn set_link(self, link: Option<String>) -> Self {
        Self { link, ..self }
    }

    /// Set optional link.
    pub fn link(self, link: impl Into<String>) -> Self {
        self.set_link(Some(link.into()))
    }

    /// Build a [`DateFormatter`] object. This method panics if token is not set.
    pub fn build(self) -> DateFormatter {
        DateFormatter {
            token: self
                .token
                .expect("token must be set to DateFormatterBuilder"),
            link: self.link,
        }
    }

    /// Get token string.
    pub fn get_token(&self) -> &Option<String> {
        &self.token
    }

    /// Get token optional link.
    pub fn get_link(&self) -> &Option<String> {
        &self.link
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_replaces_format_date_num() {
        let f = DateFormat::new(F_DATE_NUM);
        assert_eq!(f.replace_all("{date_num}"), "%Y-%m-%d");
    }

    #[test]
    fn it_replaces_format_date() {
        let f = DateFormat::new(F_DATE);
        assert_eq!(f.replace_all("{date}"), "%B %e, %Y");
    }

    #[test]
    fn it_replaces_format_date_short() {
        let f = DateFormat::new(F_DATE_SHORT);
        assert_eq!(f.replace_all("{date_short}"), "%b %e, %Y");
    }

    #[test]
    fn it_replaces_format_date_long() {
        let f = DateFormat::new(F_DATE_LONG);
        assert_eq!(f.replace_all("{date_long}"), "%A, %B %e, %Y");
    }

    #[test]
    fn it_replaces_format_date_pretty() {
        let f = DateFormat::new(F_DATE_PRETTY);
        assert_eq!(f.replace_all("{date_pretty}"), "%B %e, %Y");
    }

    #[test]
    fn it_replaces_format_date_short_pretty() {
        let f = DateFormat::new(F_DATE_SHORT_PRETTY);
        assert_eq!(f.replace_all("{date_short_pretty}"), "%b %e, %Y");
    }

    #[test]
    fn it_replaces_format_date_long_pretty() {
        let f = DateFormat::new(F_DATE_LONG_PRETTY);
        assert_eq!(f.replace_all("{date_long_pretty}"), "%A, %B %e, %Y");
    }

    #[test]
    fn it_replaces_format_time() {
        let f = DateFormat::new(F_TIME);
        assert_eq!(f.replace_all("{time}"), "%l:%M %p");
    }

    #[test]
    fn it_replaces_format_time_secs() {
        let f = DateFormat::new(F_TIME_SECS);
        assert_eq!(f.replace_all("{time_secs}"), "%l:%M:%S %p");
    }

    #[test]
    fn it_parses_multiple_formats() {
        let input = "{date} {time}";
        assert_eq!(parse_to_strf(input), "%B %e, %Y %l:%M %p");
    }

    #[test]
    fn it_formats_with_default() {
        let f = DateFormatter::default();
        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date} {time}|February 27, 2023 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_num() {
        let f = DateFormatter::builder()
            .token("{date_num} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_num} at {time}|2023-02-27 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_and_time() {
        let f = DateFormatter::builder().token("{date} at {time}").build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date} at {time}|February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_short() {
        let f = DateFormatter::builder()
            .token("{date_short} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_short} at {time}|Feb 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_long() {
        let f = DateFormatter::builder()
            .token("{date_long} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_long} at {time}|Monday, February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_pretty() {
        let f = DateFormatter::builder()
            .token("{date_pretty} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_pretty} at {time}|February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_short_pretty() {
        let f = DateFormatter::builder()
            .token("{date_short_pretty} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_short_pretty} at {time}|Feb 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_date_long_pretty() {
        let f = DateFormatter::builder()
            .token("{date_long_pretty} at {time}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date_long_pretty} at {time}|Monday, February 27, 2023 at 12:34 PM>"
        );
    }

    #[test]
    fn it_formats_with_time_secs() {
        let f = DateFormatter::builder()
            .token("{date} at {time_secs}")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date} at {time_secs}|February 27, 2023 at 12:34:56 PM>"
        );
    }

    #[test]
    fn it_formats_with_optional_link() {
        let f = DateFormatter::builder()
            .token("{date} at {time}")
            .link("https://google.com")
            .build();

        assert_eq!(
            f.format(&dt()),
            "<!date^1677468896^{date} at {time}^https://google.com|February 27, 2023 at 12:34 PM>"
        );

        assert_eq!(
            f.format_with_link(&dt(), "https://amazon.com"),
            "<!date^1677468896^{date} at {time}^https://amazon.com|February 27, 2023 at 12:34 PM>"
        );

        let f = DateFormatter::builder().token("{date} at {time}").build();

        assert_eq!(
            f.format_with_link(&dt(), "https://amazon.com"),
            "<!date^1677468896^{date} at {time}^https://amazon.com|February 27, 2023 at 12:34 PM>"
        );
    }

    fn dt() -> DateTime<FixedOffset> {
        // unix timestamp: 1677468896
        DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap()
    }
}
