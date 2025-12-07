use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, SlackFile};

use std::error::Error;
use std::fmt;

impl SlackFile {
    /// Construct a [`SlackFileBuilder`].
    pub fn builder() -> SlackFileBuilder {
        SlackFileBuilder::default()
    }
}

/// Error while building [`SlackFile`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct SlackFileError {
    /// errors of id field
    pub id: Vec<ValidationError>,

    /// errors of url filed
    pub url: Vec<ValidationError>,
}

impl fmt::Display for SlackFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "SlackFileError {{ id: {:?}, url: {:?} }}",
            self.id, self.url,
        )
    }
}

impl Error for SlackFileError {}

/// Builder for [`SlackFile`] object.
#[derive(Debug)]
pub struct SlackFileBuilder {
    id: Value<String>,
    url: Value<String>,
}

impl Default for SlackFileBuilder {
    fn default() -> Self {
        SlackFileBuilder {
            id: new_id(None),
            url: new_url(None),
        }
    }
}

impl Builder for SlackFileBuilder {
    type Target = SlackFile;
    type Error = SlackFileError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        value::merge_2(self.id, self.url)
            .map(|(id, url)| SlackFile { id, url })
            .map_err(|(id, url)| SlackFileError { id, url })
    }
}

impl SlackFileBuilder {
    /// get id field value
    pub fn get_id(&self) -> Option<&String> {
        self.id.inner_ref()
    }

    /// set id field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::SlackFile;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let file = SlackFile::builder()
    ///     .set_id(Some("F0123456"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "id": "F0123456",
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_id(self, id: Option<impl Into<String>>) -> Self {
        Self {
            id: new_id(id.map(|v| v.into())),
            ..self
        }
    }

    /// set id field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::SlackFile;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let file = SlackFile::builder()
    ///     .id("F0123456")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "id": "F0123456",
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn id(self, id: impl Into<String>) -> Self {
        self.set_id(Some(id))
    }

    /// get url field value
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::SlackFile;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let file = SlackFile::builder()
    ///     .set_url(Some("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png",
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
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
    /// use slack_messaging::composition_objects::SlackFile;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let file = SlackFile::builder()
    ///     .url("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "url": "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png",
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
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
}

fn new_id(id: Option<String>) -> Value<String> {
    pipe! { Value::new(id) => validators::do_nothing }
}

fn new_url(url: Option<String>) -> Value<String> {
    pipe! { Value::new(url) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_slack_file() {
        let result = SlackFile::builder().id("F0123456").build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = SlackFile {
            id: Some("F0123456".into()),
            url: None,
        };
        assert_eq!(val, expected);
    }
}
