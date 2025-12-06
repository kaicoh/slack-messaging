use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Conversation, ConversationFilter};

use std::error::Error;
use std::fmt;

impl ConversationFilter {
    /// Construct a [`ConversationFilter`].
    pub fn builder() -> ConversationFilterBuilder {
        ConversationFilterBuilder::default()
    }
}

/// Error while building [`ConversationFilter`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct ConversationFilterError {
    /// errors of include field
    pub include: Vec<ValidationError>,

    /// errors of exclude_external_shared_channels field
    pub exclude_external_shared_channels: Vec<ValidationError>,

    /// errors of exclude_bot_users field
    pub exclude_bot_users: Vec<ValidationError>,
}

impl fmt::Display for ConversationFilterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConversationFilterError {{ include: {:?}, exclude_external_shared_channels: {:?}, exclude_bot_users: {:?} }}",
            self.include, self.exclude_external_shared_channels, self.exclude_bot_users,
        )
    }
}

impl Error for ConversationFilterError {}

/// Builder for [`ConversationFilter`] object.
#[derive(Debug)]
pub struct ConversationFilterBuilder {
    include: Value<Vec<Conversation>>,
    exclude_external_shared_channels: Value<bool>,
    exclude_bot_users: Value<bool>,
}

impl Default for ConversationFilterBuilder {
    fn default() -> Self {
        ConversationFilterBuilder {
            include: new_include(None),
            exclude_external_shared_channels: new_exclude_external_shared_channels(None),
            exclude_bot_users: new_exclude_bot_users(None),
        }
    }
}

impl Builder for ConversationFilterBuilder {
    type Target = ConversationFilter;
    type Error = ConversationFilterError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            include,
            exclude_external_shared_channels,
            exclude_bot_users,
        } = self;

        value::merge_3(include, exclude_external_shared_channels, exclude_bot_users)
            .map(
                |(include, exclude_external_shared_channels, exclude_bot_users)| {
                    ConversationFilter {
                        include: include.unwrap_or_default(),
                        exclude_external_shared_channels,
                        exclude_bot_users,
                    }
                },
            )
            .map_err(
                |(include, exclude_external_shared_channels, exclude_bot_users)| {
                    ConversationFilterError {
                        include,
                        exclude_external_shared_channels,
                        exclude_bot_users,
                    }
                },
            )
    }
}

impl ConversationFilterBuilder {
    /// get include field value
    pub fn get_include(&self) -> Option<&[Conversation]> {
        self.include.inner_ref().map(|v| v.as_ref())
    }

    /// set include field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .set_include(
    ///         vec![
    ///             Conversation::Im,
    ///             Conversation::Private,
    ///         ]
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "include": [
    ///         "im",
    ///         "private"
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_include(self, include: Vec<Conversation>) -> Self {
        Self {
            include: new_include(Some(include)),
            ..self
        }
    }

    /// add value to include field
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::{Conversation, ConversationFilter};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .include(Conversation::Im)
    ///     .include(Conversation::Private)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "include": [
    ///         "im",
    ///         "private"
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn include(mut self, include: Conversation) -> Self {
        let mut list = self.include.take_inner().unwrap_or_default();
        list.push(include);
        self.set_include(list)
    }

    /// get exclude_external_shared_channels field value
    pub fn get_exclude_external_shared_channels(&self) -> Option<bool> {
        self.exclude_external_shared_channels.inner_ref().copied()
    }

    /// set exclude_external_shared_channels field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::ConversationFilter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .set_exclude_external_shared_channels(Some(true))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_external_shared_channels": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_exclude_external_shared_channels(self, value: Option<bool>) -> Self {
        Self {
            exclude_external_shared_channels: new_exclude_external_shared_channels(value),
            ..self
        }
    }

    /// set exclude_external_shared_channels field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::ConversationFilter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .exclude_external_shared_channels(false)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_external_shared_channels": false
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn exclude_external_shared_channels(self, value: bool) -> Self {
        self.set_exclude_external_shared_channels(Some(value))
    }

    /// get exclude_bot_users field value
    pub fn get_exclude_bot_users(&self) -> Option<bool> {
        self.exclude_bot_users.inner_ref().copied()
    }

    /// set exclude_bot_users field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::ConversationFilter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .set_exclude_bot_users(Some(true))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_bot_users": true
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_exclude_bot_users(self, exclude_bot_users: Option<bool>) -> Self {
        Self {
            exclude_bot_users: new_exclude_bot_users(exclude_bot_users),
            ..self
        }
    }

    /// set exclude_bot_users field value
    ///
    /// ```
    /// use slack_messaging::Builder;
    /// use slack_messaging::composition_objects::ConversationFilter;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let filter = ConversationFilter::builder()
    ///     .exclude_bot_users(false)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "exclude_bot_users": false
    /// });
    ///
    /// let json = serde_json::to_value(filter).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn exclude_bot_users(self, exclude_bot_users: bool) -> Self {
        self.set_exclude_bot_users(Some(exclude_bot_users))
    }
}

fn new_include(include: Option<Vec<Conversation>>) -> Value<Vec<Conversation>> {
    pipe! { Value::new(include) => validators::do_nothing }
}

fn new_exclude_external_shared_channels(
    exclude_external_shared_channels: Option<bool>,
) -> Value<bool> {
    pipe! {
        Value::new(exclude_external_shared_channels) =>
            validators::do_nothing
    }
}

fn new_exclude_bot_users(exclude_bot_users: Option<bool>) -> Value<bool> {
    pipe! { Value::new(exclude_bot_users) => validators::do_nothing }
}
