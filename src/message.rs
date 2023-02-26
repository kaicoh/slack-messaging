use super::{Attachment, blocks::Block};
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Message {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    attachments: Vec<Attachment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    mrkdwn: Option<bool>,
}

impl Message {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }

    pub fn push_block<T: Into<Block>>(self, block: T) -> Self {
        let Self { mut blocks, .. } = self;
        blocks.push(block.into());
        Self { blocks, ..self }
    }

    pub fn push_attachment(self, attachment: Attachment) -> Self {
        let Self {
            mut attachments, ..
        } = self;
        attachments.push(attachment);
        Self {
            attachments,
            ..self
        }
    }

    pub fn attach(self, attachment: Attachment) -> Self {
        self.push_attachment(attachment)
    }

    pub fn set_thread_ts<T: Into<String>>(self, thread_ts: T) -> Self {
        Self {
            thread_ts: Some(thread_ts.into()),
            ..self
        }
    }

    pub fn set_mrkdwn(self, mrkdwn: bool) -> Self {
        Self {
            mrkdwn: Some(mrkdwn),
            ..self
        }
    }
}
