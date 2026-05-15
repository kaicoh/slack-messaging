use crate::blocks::TaskCard;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Plan block](https://docs.slack.dev/reference/block-kit/blocks/plan-block) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official documentation](https://docs.slack.dev/reference/block-kit/blocks/plan-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | title | String | Yes | N/A |
/// | block_id | String | No | Maximum 255 characters |
/// | tasks | Vec<[TaskCard]> | No | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::{Plan, RichText, TaskCard, TaskStatus};
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::RichTextElementText;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let plan = Plan::builder()
///     .title("Thinking completed")
///     .task(
///         TaskCard::builder()
///             .task_id("call_001")
///             .title("Fetched user profile information")
///             .status(TaskStatus::InProgress)
///             .details(
///                 RichText::builder()
///                     .block_id("viMWO")
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("Searched database...")
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .output(
///                 RichText::builder()
///                     .block_id("viMWO")
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("Profile data loaded")
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .task(
///         TaskCard::builder()
///             .task_id("call_002")
///             .title("Checked user permissions")
///             .status(TaskStatus::Pending)
///             .build()?
///     )
///     .task(
///         TaskCard::builder()
///             .task_id("call_003")
///             .title("Generated comprehensive user report")
///             .status(TaskStatus::Complete)
///             .output(
///                 RichText::builder()
///                     .block_id("crsk")
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("15 data points compiled")
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///   "type": "plan",
///   "title": "Thinking completed",
///   "tasks": [
///     {
///       "type": "task_card",
///       "task_id": "call_001",
///       "title": "Fetched user profile information",
///       "status": "in_progress",
///       "details": {
///         "type": "rich_text",
///         "block_id": "viMWO",
///         "elements": [
///           {
///             "type": "rich_text_section",
///             "elements": [
///               {
///                 "type": "text",
///                 "text": "Searched database..."
///               }
///             ]
///           }
///         ]
///       },
///       "output": {
///         "type": "rich_text",
///         "block_id": "viMWO",
///         "elements": [
///           {
///             "type": "rich_text_section",
///             "elements": [
///               {
///                 "type": "text",
///                 "text": "Profile data loaded"
///               }
///             ]
///           }
///         ]
///       }
///     },
///     {
///       "type": "task_card",
///       "task_id": "call_002",
///       "title": "Checked user permissions",
///       "status": "pending"
///     },
///     {
///       "type": "task_card",
///       "task_id": "call_003",
///       "title": "Generated comprehensive user report",
///       "status": "complete",
///       "output": {
///         "type": "rich_text",
///         "block_id": "crsk",
///         "elements": [
///           {
///             "type": "rich_text_section",
///             "elements": [
///               {
///                 "type": "text",
///                 "text": "15 data points compiled"
///               }
///             ]
///           }
///         ]
///       }
///     }
///   ]
/// });
///
/// let json = serde_json::to_value(plan).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
///```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "plan")]
pub struct Plan {
    #[builder(validate("required"))]
    pub(crate) title: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "task")]
    pub(crate) tasks: Option<Vec<TaskCard>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Plan {
            title: Some("Thinking completed".into()),
            block_id: Some("block_0".into()),
            tasks: Some(vec![task_card()]),
        };

        let val = Plan::builder()
            .set_title(Some("Thinking completed"))
            .set_block_id(Some("block_0"))
            .set_tasks(Some(vec![task_card()]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Plan::builder()
            .title("Thinking completed")
            .block_id("block_0")
            .tasks(vec![task_card()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Plan {
            title: Some("Thinking completed".into()),
            block_id: None,
            tasks: Some(vec![task_card()]),
        };

        let val = Plan::builder()
            .title("Thinking completed")
            .task(task_card())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_title_field() {
        let err = Plan::builder().build().unwrap_err();
        assert_eq!(err.object(), "Plan");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Plan::builder()
            .title("Thinking completed")
            .block_id("b".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Plan");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
