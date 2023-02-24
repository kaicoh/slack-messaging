use super::elements::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Video {
    #[serde(rename = "type")]
    kind: &'static str,

    alt_text: String,

    title: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    title_url: Option<String>,

    thumbnail_url: String,

    video_url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    author_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    provider_icon_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    provider_name: Option<String>,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            kind: "video",
            alt_text: "".to_string(),
            title: Text::plain(""),
            title_url: None,
            thumbnail_url: "".to_string(),
            video_url: "".to_string(),
            author_name: None,
            block_id: None,
            description: None,
            provider_icon_url: None,
            provider_name: None,
        }
    }
}

impl Video {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_alt_text<T: Into<String>>(self, alt: T) -> Self {
        Self {
            alt_text: alt.into(),
            ..self
        }
    }

    pub fn set_title(self, title: Text) -> Self {
        Self { title, ..self }
    }

    pub fn title<T: Into<String>>(self, title: T) -> Self {
        self.set_title(Text::plain(title))
    }

    pub fn set_thumbnail_url<T: Into<String>>(self, thumbnail_url: T) -> Self {
        Self {
            thumbnail_url: thumbnail_url.into(),
            ..self
        }
    }

    pub fn set_video_url<T: Into<String>>(self, video_url: T) -> Self {
        Self {
            video_url: video_url.into(),
            ..self
        }
    }

    pub fn set_author_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            author_name: Some(name.into()),
            ..self
        }
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }

    pub fn set_description(self, description: Text) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    pub fn description<T: Into<String>>(self, description: T) -> Self {
        self.set_description(Text::plain(description))
    }

    pub fn set_provider_icon_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            provider_icon_url: Some(url.into()),
            ..self
        }
    }

    pub fn set_provider_name<T: Into<String>>(self, name: T) -> Self {
        Self {
            provider_name: Some(name.into()),
            ..self
        }
    }
}
