use axum::response::sse::Event;
use std::convert::Infallible;

use super::openai_types::ChatCompletionChunk;

pub struct SseEvent(pub ChatCompletionChunk);

impl From<SseEvent> for Result<Event, Infallible> {
    fn from(event: SseEvent) -> Self {
        let chunk = event.0;
        let json = serde_json::to_string(&chunk).unwrap();
        Ok(Event::default().data(json))
    }
}

pub fn create_done_event() -> Result<Event, Infallible> {
    Ok(Event::default().data("[DONE]"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::openai_types::{ChatCompletionChunk, ChunkChoice, Delta};

    #[test]
    fn test_sse_event_conversion() {
        let chunk = ChatCompletionChunk {
            id: "test-id".to_string(),
            object: "chat.completion.chunk".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![ChunkChoice {
                index: 0,
                delta: Delta {
                    role: Some("assistant".to_string()),
                    content: Some("Hello".to_string()),
                },
                finish_reason: None,
            }],
        };

        let sse_event = SseEvent(chunk);
        let result: Result<Event, Infallible> = sse_event.into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_sse_event_with_empty_choices() {
        let chunk = ChatCompletionChunk {
            id: "empty-choices-id".to_string(),
            object: "chat.completion.chunk".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![],
        };

        let sse_event = SseEvent(chunk);
        let result: Result<Event, Infallible> = sse_event.into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_sse_event_with_multiple_choices() {
        let chunk = ChatCompletionChunk {
            id: "multiple-choices-id".to_string(),
            object: "chat.completion.chunk".to_string(),
            created: 1234567890,
            model: "test-model".to_string(),
            choices: vec![
                ChunkChoice {
                    index: 0,
                    delta: Delta {
                        role: Some("assistant".to_string()),
                        content: Some("Hello".to_string()),
                    },
                    finish_reason: None,
                },
                ChunkChoice {
                    index: 1,
                    delta: Delta {
                        role: None,
                        content: Some("World".to_string()),
                    },
                    finish_reason: Some("stop".to_string()),
                },
            ],
        };

        let sse_event = SseEvent(chunk);
        let result: Result<Event, Infallible> = sse_event.into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_sse_event_json_serialization() {
        let chunk = ChatCompletionChunk {
            id: "json-test-id".to_string(),
            object: "chat.completion.chunk".to_string(),
            created: 1234567890,
            model: "json-test-model".to_string(),
            choices: vec![ChunkChoice {
                index: 0,
                delta: Delta {
                    role: Some("assistant".to_string()),
                    content: Some("Test content".to_string()),
                },
                finish_reason: Some("stop".to_string()),
            }],
        };

        let json = serde_json::to_string(&chunk).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["id"], "json-test-id");
        assert_eq!(parsed["object"], "chat.completion.chunk");
        assert_eq!(parsed["model"], "json-test-model");
        assert_eq!(parsed["choices"][0]["delta"]["content"], "Test content");

        let sse_event = SseEvent(chunk);
        let result: Result<Event, Infallible> = sse_event.into();
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_done_event() {
        let result = create_done_event();
        assert!(result.is_ok());
    }
}
