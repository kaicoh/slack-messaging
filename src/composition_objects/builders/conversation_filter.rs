use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Conversation, ConversationFilter};

use std::error::Error;
use std::fmt;

impl ConversationFilter {
    /// Construct a [`ConversationFilterBuilder`].
    pub fn builder() -> ConversationFilterBuilder {
        ConversationFilterBuilder::default()
    }
}

/// Error while building [`ConversationFilter`] object.
#[derive(Debug, Clone, PartialEq, Default)]
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
            mut include,
            mut exclude_external_shared_channels,
            mut exclude_bot_users,
        } = self;

        if include.inner_ref().is_none()
            && exclude_external_shared_channels.inner_ref().is_none()
            && exclude_bot_users.inner_ref().is_none()
        {
            let err = ValidationError::NoFieldProvided;
            include.push(err);
            exclude_external_shared_channels.push(err);
            exclude_bot_users.push(err);
        }

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
    pub fn set_include(self, include: Option<Vec<Conversation>>) -> Self {
        Self {
            include: new_include(include),
            ..self
        }
    }

    /// set include field value
    pub fn include(self, include: Vec<Conversation>) -> Self {
        self.set_include(Some(include))
    }

    /// add conversation to include field
    pub fn conversation(mut self, conversation: Conversation) -> Self {
        let mut list = self.include.take_inner().unwrap_or_default();
        list.push(conversation);
        self.include(list)
    }

    /// get exclude_external_shared_channels field value
    pub fn get_exclude_external_shared_channels(&self) -> Option<bool> {
        self.exclude_external_shared_channels.inner_ref().copied()
    }

    /// set exclude_external_shared_channels field value
    pub fn set_exclude_external_shared_channels(self, value: Option<bool>) -> Self {
        Self {
            exclude_external_shared_channels: new_exclude_external_shared_channels(value),
            ..self
        }
    }

    /// set exclude_external_shared_channels field value
    pub fn exclude_external_shared_channels(self, value: bool) -> Self {
        self.set_exclude_external_shared_channels(Some(value))
    }

    /// get exclude_bot_users field value
    pub fn get_exclude_bot_users(&self) -> Option<bool> {
        self.exclude_bot_users.inner_ref().copied()
    }

    /// set exclude_bot_users field value
    pub fn set_exclude_bot_users(self, exclude_bot_users: Option<bool>) -> Self {
        Self {
            exclude_bot_users: new_exclude_bot_users(exclude_bot_users),
            ..self
        }
    }

    /// set exclude_bot_users field value
    pub fn exclude_bot_users(self, exclude_bot_users: bool) -> Self {
        self.set_exclude_bot_users(Some(exclude_bot_users))
    }
}

fn new_include(include: Option<Vec<Conversation>>) -> Value<Vec<Conversation>> {
    pipe! { Value::new(include) => validators::list::not_empty }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let val = ConversationFilter::builder()
            .set_include(Some(vec![Conversation::Im, Conversation::Private]))
            .set_exclude_external_shared_channels(Some(true))
            .set_exclude_bot_users(Some(true))
            .build()
            .unwrap();

        let expected = ConversationFilter {
            include: vec![Conversation::Im, Conversation::Private],
            exclude_external_shared_channels: Some(true),
            exclude_bot_users: Some(true),
        };
        assert_eq!(val, expected);

        let val = ConversationFilter::builder()
            .include(vec![Conversation::Im, Conversation::Private])
            .exclude_external_shared_channels(false)
            .exclude_bot_users(false)
            .build()
            .unwrap();

        let expected = ConversationFilter {
            include: vec![Conversation::Im, Conversation::Private],
            exclude_external_shared_channels: Some(false),
            exclude_bot_users: Some(false),
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn it_has_additional_setter_for_include_field() {
        let val = ConversationFilter::builder()
            .conversation(Conversation::Im)
            .conversation(Conversation::Private)
            .build()
            .unwrap();

        let expected = ConversationFilter {
            include: vec![Conversation::Im, Conversation::Private],
            exclude_external_shared_channels: None,
            exclude_bot_users: None,
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn include_field_cannot_be_empty() {
        let err = ConversationFilter::builder()
            .include(vec![])
            .build()
            .unwrap_err();

        let expected = ConversationFilterError {
            include: vec![ValidationError::EmptyArray],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn it_sets_error_if_no_field_provided() {
        let err = ConversationFilter::builder().build().unwrap_err();

        let expected = ConversationFilterError {
            include: vec![ValidationError::NoFieldProvided],
            exclude_external_shared_channels: vec![ValidationError::NoFieldProvided],
            exclude_bot_users: vec![ValidationError::NoFieldProvided],
        };

        assert_eq!(err, expected);
    }
}
