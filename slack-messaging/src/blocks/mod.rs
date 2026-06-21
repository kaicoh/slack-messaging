use serde::Serialize;

/// Builders for blocks.
pub mod builders;
/// Objects from which blocks are composed.
pub mod elements;
/// Module for building [RichText] block.
pub mod rich_text;
/// Module for building [Table] block.
pub mod table;
/// Module for building [DataVisualization] block.
pub mod data_visualization;
/// Module for building [DataTable] block.
pub mod data_table;

mod actions;
mod alert;
mod card;
mod carousel;
mod context;
mod context_actions;
mod divider;
mod file;
mod header;
mod image;
mod input;
mod markdown;
mod plan;
mod section;
mod task_card;
mod video;

pub use actions::{Actions, ActionsElement};
pub use alert::{Alert, AlertLevel};
pub use card::Card;
pub use carousel::Carousel;
pub use context::{Context, ContextElement};
pub use context_actions::{ContextActions, ContextActionsElement};
pub use data_table::DataTable;
pub use data_visualization::DataVisualization;
pub use divider::Divider;
pub use file::{File, FileSource};
pub use header::Header;
pub use image::Image;
pub use input::{Input, InputElement};
pub use markdown::Markdown;
pub use plan::Plan;
pub use rich_text::RichText;
pub use section::{Accessory, Section};
pub use table::Table;
pub use task_card::{TaskCard, TaskStatus};
pub use video::Video;

/// Objects that can be set to blocks in [Message](crate::message::Message).
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Block {
    /// [Actions block](https://docs.slack.dev/reference/block-kit/blocks/actions-block) representation
    Actions(Box<Actions>),

    /// [Alert block](https://docs.slack.dev/reference/block-kit/blocks/alert-block) representation
    Alert(Box<Alert>),

    /// [Card block](https://docs.slack.dev/reference/block-kit/blocks/card-block)
    /// representation
    Card(Box<Card>),

    /// [Carousel block](https://docs.slack.dev/reference/block-kit/blocks/carousel-block)
    /// representation
    Carousel(Box<Carousel>),

    /// [Context block](https://docs.slack.dev/reference/block-kit/blocks/context-block) representation
    Context(Box<Context>),

    /// [Context actions block](https://docs.slack.dev/reference/block-kit/blocks/context-actions-block) representation
    ContextActions(Box<ContextActions>),

    /// [Data table block](https://docs.slack.dev/reference/block-kit/blocks/data-table-block) representation
    DataTable(Box<DataTable>),
 
    /// [Data visualization
    /// block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block)
    /// representation
    DataVisualization(Box<DataVisualization>),

    /// [Divider block](https://docs.slack.dev/reference/block-kit/blocks/divider-block) representation
    Divider(Box<Divider>),

    /// [File block](https://docs.slack.dev/reference/block-kit/blocks/file-block) representation
    File(Box<File>),

    /// [Header block](https://docs.slack.dev/reference/block-kit/blocks/header-block) representation
    Header(Box<Header>),

    /// [Image block](https://docs.slack.dev/reference/block-kit/blocks/image-block) representation
    Image(Box<Image>),

    /// [Input block](https://docs.slack.dev/reference/block-kit/blocks/input-block) representation
    Input(Box<Input>),

    /// [Markdown block](https://docs.slack.dev/reference/block-kit/blocks/markdown-block) representation
    Markdown(Box<Markdown>),

    /// [Plan block](https://docs.slack.dev/reference/block-kit/blocks/plan-block) representation
    Plan(Box<Plan>),

    /// [Rich text block](https://docs.slack.dev/reference/block-kit/blocks/rich-text-block) representation
    RichText(Box<RichText>),

    /// [Section block](https://docs.slack.dev/reference/block-kit/blocks/section-block) representation
    Section(Box<Section>),

    /// [Table block](https://docs.slack.dev/reference/block-kit/blocks/table-block) representation
    Table(Box<Table>),

    /// [Task card block](https://docs.slack.dev/reference/block-kit/blocks/task-card-block)
    /// representation
    TaskCard(Box<TaskCard>),

    /// [Video block](https://docs.slack.dev/reference/block-kit/blocks/video-block) representation
    Video(Box<Video>),
}

macro_rules! block_from {
    ($($ty:ident,)*) => {
        $(
            impl From<$ty> for Block {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

block_from! {
    Actions,
    Alert,
    Card,
    Carousel,
    Context,
    ContextActions,
    DataTable,
    DataVisualization,
    Divider,
    File,
    Header,
    Image,
    Input,
    Markdown,
    Plan,
    RichText,
    Section,
    Table,
    TaskCard,
    Video,
}

#[cfg(test)]
pub mod test_helpers {
    use super::rich_text::test_helpers as rich_text_helper;
    use super::rich_text::types::test_helpers::*;
    use super::*;
    use crate::composition_objects::test_helpers::*;

    pub fn header(text: impl Into<String>) -> Header {
        Header {
            block_id: None,
            text: Some(plain_text(text)),
        }
    }

    pub fn section(text: impl Into<String>) -> Section {
        Section {
            block_id: None,
            text: Some(mrkdwn_text(text).into()),
            fields: None,
            accessory: None,
            expand: None,
        }
    }

    pub fn rich_text() -> RichText {
        RichText {
            block_id: Some("rich_text_0".into()),
            elements: Some(vec![rich_text_helper::section(vec![el_text("foo")]).into()]),
        }
    }

    pub fn task_card() -> TaskCard {
        TaskCard {
            task_id: Some("task_0".into()),
            title: Some("Fetching weather data".into()),
            status: Some(TaskStatus::Pending),
            output: None,
            details: None,
            sources: None,
            block_id: None,
        }
    }
}
