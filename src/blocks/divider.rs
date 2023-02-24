use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            kind: "divider",
            block_id: None,
        }
    }
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
