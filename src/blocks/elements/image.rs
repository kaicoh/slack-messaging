use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Image {
    #[serde(rename = "type")]
    kind: &'static str,

    image_url: String,

    alt_text: String,
}

impl Image {
    pub fn new<S, T>(alt: S, url: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        Self {
            kind: "image",
            image_url: url.into(),
            alt_text: alt.into(),
        }
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
}
