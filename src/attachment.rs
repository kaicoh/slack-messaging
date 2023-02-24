use super::Block;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<String>,
}

impl Attachment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_color<T: Into<String>>(self, color: T) -> Self {
        Self {
            color: Some(color.into()),
            ..self
        }
    }

    pub fn push_block<T: Into<Block>>(self, block: T) -> Self {
        let Self { mut blocks, .. } = self;
        blocks.push(block.into());
        Self { blocks, ..self }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sets_color() {
        let attachment = Attachment::new().set_color("#FFFFFF");
        assert_eq!(attachment.color, Some("#FFFFFF".into()));
    }
}
