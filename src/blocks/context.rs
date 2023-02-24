use super::elements::{Image, Text};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Context {
    #[serde(rename = "type")]
    kind: &'static str,

    elements: Vec<ContextElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            kind: "context",
            elements: vec![],
            block_id: None,
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_elements(self, elements: Vec<ContextElement>) -> Self {
        Self { elements, ..self }
    }

    pub fn push_element<T: Into<ContextElement>>(self, element: T) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ContextElement {
    Image(Box<Image>),
    Text(Box<Text>),
}

impl From<Image> for ContextElement {
    fn from(value: Image) -> Self {
        Self::Image(Box::new(value))
    }
}

impl From<Text> for ContextElement {
    fn from(value: Text) -> Self {
        Self::Text(Box::new(value))
    }
}
