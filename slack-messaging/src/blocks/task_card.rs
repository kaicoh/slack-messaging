use crate::blocks::RichText;
use crate::blocks::elements::UrlSource;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Task card](https://docs.slack.dev/reference/block-kit/blocks/task-card-block) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official documentation](https://docs.slack.dev/reference/block-kit/blocks/task-card-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | task_id | String | Yes | N/A |
/// | title | String | Yes | N/A |
/// | details | [RichText] | No | Have exactly one element |
/// | output | [RichText] | No | Have exactly one element |
/// | sources | Vec<[UrlSource]> | No | N/A |
/// | status | [TaskStatus] | No | N/A |
/// | block_id | String | No | Maximum 255 characters |
///
/// # Example
///
/// The following is reproduction of [the sample task
/// card](https://docs.slack.dev/reference/block-kit/blocks/task-card-block#examples).
///
/// ```
/// use slack_messaging::blocks::{RichText, TaskCard, TaskStatus};
/// use slack_messaging::blocks::elements::UrlSource;
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::RichTextElementText;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let task_card = TaskCard::builder()
///     .task_id("task_1")
///     .title("Fetching weather data")
///     .status(TaskStatus::Pending)
///     .output(
///         RichText::builder()
///             .element(
///                 RichTextSection::builder()
///                     .element(
///                         RichTextElementText::builder()
///                             .text("Found weather data for Chicago from 2 sources")
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .source(
///         UrlSource::builder()
///             .url("https://weather.com/")
///             .text("weather.com")
///             .build()?
///     )
///     .source(
///         UrlSource::builder()
///             .url("https://www.accuweather.com/")
///             .text("accuweather.com")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///    "type": "task_card",
///    "task_id": "task_1",
///    "title": "Fetching weather data",
///    "status": "pending",
///    "output": {
///        "type": "rich_text",
///        "elements": [
///            {
///                "type": "rich_text_section",
///                "elements": [
///                    {
///                        "type": "text",
///                        "text": "Found weather data for Chicago from 2 sources"
///                    }
///                ]
///            }
///        ]
///    },
///    "sources": [
///        {
///            "type": "url",
///            "url": "https://weather.com/",
///            "text": "weather.com"
///        },
///        {
///            "type": "url",
///            "url": "https://www.accuweather.com/",
///            "text": "accuweather.com"
///        }
///    ]
/// });
///
/// let json = serde_json::to_value(task_card).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "task_card")]
pub struct TaskCard {
    #[builder(validate("required"))]
    pub(crate) task_id: Option<String>,

    #[builder(validate("required"))]
    pub(crate) title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("rich_text::single_element"))]
    pub(crate) details: Option<RichText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("rich_text::single_element"))]
    pub(crate) output: Option<RichText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "source")]
    pub(crate) sources: Option<Vec<UrlSource>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) status: Option<TaskStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

/// Values that can be set to the status field of [TaskCard].
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Complete,
    Error,
}

#[cfg(test)]
mod tests {
    use super::super::rich_text::{
        RichText, test_helpers::section, types::RichTextElementType, types::test_helpers::el_text,
    };
    use super::*;
    use crate::blocks::rich_text::RichTextSubElement;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let output = rich_text(vec![el_text("output of task card")]);
        let details = rich_text(vec![el_text("details for task card")]);
        let sources = vec![
            url_source("https://weather.com/", "weather.com"),
            url_source("https://www.accuweather.com/", "accuweather.com"),
        ];

        let expected = TaskCard {
            task_id: Some("task_1".into()),
            title: Some("Fetching weather data".into()),
            status: Some(TaskStatus::Pending),
            output: Some(output.clone()),
            details: Some(details.clone()),
            sources: Some(sources.clone()),
            block_id: Some("task_card_0".into()),
        };

        let val = TaskCard::builder()
            .set_task_id(Some("task_1"))
            .set_title(Some("Fetching weather data"))
            .set_status(Some(TaskStatus::Pending))
            .set_output(Some(output.clone()))
            .set_details(Some(details.clone()))
            .set_sources(Some(sources.clone()))
            .set_block_id(Some("task_card_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = TaskCard::builder()
            .task_id("task_1")
            .title("Fetching weather data")
            .status(TaskStatus::Pending)
            .output(output.clone())
            .details(details.clone())
            .sources(sources.clone())
            .block_id("task_card_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = TaskCard {
            task_id: Some("task_1".into()),
            title: Some("Fetching weather data".into()),
            status: None,
            output: None,
            details: None,
            sources: Some(vec![
                url_source("https://weather.com/", "weather.com"),
                url_source("https://www.accuweather.com/", "accuweather.com"),
            ]),
            block_id: None,
        };

        let val = TaskCard::builder()
            .task_id("task_1")
            .title("Fetching weather data")
            .source(url_source("https://weather.com/", "weather.com"))
            .source(url_source(
                "https://www.accuweather.com/",
                "accuweather.com",
            ))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_task_id_field() {
        let err = TaskCard::builder().title("foo").build().unwrap_err();
        assert_eq!(err.object(), "TaskCard");

        let errors = err.field("task_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_title_field() {
        let err = TaskCard::builder().task_id("task_1").build().unwrap_err();
        assert_eq!(err.object(), "TaskCard");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_details_and_output_fields_to_have_exactly_one_element() {
        let err = TaskCard::builder()
            .task_id("task_1")
            .title("foo")
            .details(rich_text(vec![]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TaskCard");

        let errors = err.field("details");
        assert!(errors.includes(ValidationErrorKind::RichTextSingleElement));

        let err = TaskCard::builder()
            .task_id("task_1")
            .title("foo")
            .output(rich_text(vec![]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TaskCard");

        let errors = err.field("output");
        assert!(errors.includes(ValidationErrorKind::RichTextSingleElement));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = TaskCard::builder()
            .task_id("task_1")
            .title("foo")
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "TaskCard");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    fn rich_text(texts: Vec<RichTextElementType>) -> RichText {
        RichText {
            block_id: None,
            elements: Some(
                texts
                    .into_iter()
                    .map(|text| section(vec![text]))
                    .map(RichTextSubElement::from)
                    .collect(),
            ),
        }
    }

    fn url_source(url: &str, text: &str) -> UrlSource {
        UrlSource {
            url: Some(url.into()),
            text: Some(text.into()),
        }
    }
}
