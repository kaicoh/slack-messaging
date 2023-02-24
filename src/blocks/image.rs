use super::elements::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    kind: &'static str,

    image_url: String,

    alt_text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            kind: "image",
            image_url: "".to_string(),
            alt_text: "".to_string(),
            title: None,
            block_id: None,
        }
    }
}

impl Image {
    pub fn new<S, T>(alt: S, url: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self::default().set_image_url(url).set_alt_text(alt)
    }

    pub fn set_image_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            image_url: url.into(),
            ..self
        }
    }

    pub fn set_alt_text<T: Into<String>>(self, alt: T) -> Self {
        Self {
            alt_text: alt.into(),
            ..self
        }
    }

    pub fn set_title(self, text: Text) -> Self {
        Self {
            title: Some(text),
            ..self
        }
    }

    pub fn title<T: Into<String>>(self, title: T) -> Self {
        self.set_title(Text::plain(title))
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
