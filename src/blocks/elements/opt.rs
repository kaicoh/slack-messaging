use super::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Opt {
    text: Text,

    value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Opt {
    pub fn plain<S, T>(text: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            text: Text::plain(text),
            value: value.into(),
            description: None,
            url: None,
        }
    }
    pub fn mrkdwn<S, T>(text: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            text: Text::mrkdwn(text),
            value: value.into(),
            description: None,
            url: None,
        }
    }

    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    pub fn set_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }

    pub fn set_description<T: Into<String>>(self, description: T) -> Self {
        Self {
            description: Some(Text::plain(description)),
            ..self
        }
    }

    pub fn set_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }
}
