use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Conversation {
    Im,
    Mpim,
    Private,
    Public,
}

#[derive(Debug, Default, Serialize)]
pub struct Filter {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    include: Vec<Conversation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_external_shared_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    exclude_bot_users: Option<bool>,
}

impl Filter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_include(self, include: Vec<Conversation>) -> Self {
        Self { include, ..self }
    }

    pub fn push_include(self, conversation: Conversation) -> Self {
        let Self { mut include, .. } = self;
        include.push(conversation);
        Self { include, ..self }
    }

    pub fn include(self, conversation: Conversation) -> Self {
        self.push_include(conversation)
    }

    pub fn set_exclude_external_shared_channels(self, value: bool) -> Self {
        Self {
            exclude_external_shared_channels: Some(value),
            ..self
        }
    }

    pub fn exclude_external_shared_channels(self) -> Self {
        self.set_exclude_external_shared_channels(true)
    }

    pub fn set_exclude_bot_users(self, value: bool) -> Self {
        Self {
            exclude_bot_users: Some(value),
            ..self
        }
    }

    pub fn exclude_bot_users(self) -> Self {
        self.set_exclude_bot_users(true)
    }
}
