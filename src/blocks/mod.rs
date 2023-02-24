mod elements;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Block {
    Actions,
    Context,
    Divider,
    File,
    Header,
    Image,
    Input,
    Section,
    Video,
}
