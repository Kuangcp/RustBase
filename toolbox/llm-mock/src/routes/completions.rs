use axum::extract::State;
use axum::response::sse::{Event, Sse};
use axum::Json;
use futures::stream::Stream;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

use crate::error::AppError;
use crate::models::stream::create_done_event;
use crate::AppState;

#[derive(Debug, serde::Deserialize)]
pub struct TextCompletionRequest {
    pub model: String,
    pub prompt: Option<String>,
    pub stream: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
pub struct TextCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<TextChoice>,
}

#[derive(Debug, serde::Serialize)]
pub struct TextChoice {
    pub text: String,
    pub index: u32,
    pub finish_reason: Option<String>,
}

pub async fn text_completions(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TextCompletionRequest>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    check_error_simulation(&state)?;

    let (tx, rx) = mpsc::channel(32);
    let content = state.response_content.clone();
    let config = state.config.clone();
    let completion_id = format!("cmpl-{}", uuid::Uuid::new_v4());
    let model = payload.model.clone();
    let stream = payload.stream.unwrap_or(false);

    if stream {
        tokio::spawn(async move {
            let tokens = tokenize(&content);

            for token in tokens {
                let response = TextCompletionResponse {
                    id: completion_id.clone(),
                    object: "text_completion".to_string(),
                    created: chrono::Utc::now().timestamp(),
                    model: model.clone(),
                    choices: vec![TextChoice {
                        text: token,
                        index: 0,
                        finish_reason: None,
                    }],
                };

                let _ = tx
                    .send(Ok(Event::default().data(serde_json::to_string(&response).unwrap())))
                    .await;

                let delay = rand::random::<u64>()
                    % (config.response.stream_delay_max_ms - config.response.stream_delay_min_ms + 1)
                    + config.response.stream_delay_min_ms;
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }

            let _ = tx.send(create_done_event()).await;
        });
    } else {
        let response = TextCompletionResponse {
            id: completion_id,
            object: "text_completion".to_string(),
            created: chrono::Utc::now().timestamp(),
            model,
            choices: vec![TextChoice {
                text: content,
                index: 0,
                finish_reason: Some("stop".to_string()),
            }],
        };
        let _ = tx
            .send(Ok(Event::default().data(serde_json::to_string(&response).unwrap())))
            .await;
        let _ = tx.send(create_done_event()).await;
    }

    Ok(Sse::new(ReceiverStream::new(rx)))
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == '!')
        .filter(|s| !s.is_empty())
        .map(|s| {
            if text.contains(&format!("{} ", s)) {
                format!("{} ", s)
            } else {
                s.to_string()
            }
        })
        .collect()
}

fn check_error_simulation(state: &Arc<AppState>) -> Result<(), AppError> {
    if !state.config.error_simulation.enabled {
        return Ok(());
    }

    let mut rng = rand::thread_rng();
    use rand::Rng;

    if rng.gen_bool(state.config.error_simulation.rate_limit_probability) {
        return Err(AppError::RateLimit);
    }

    if rng.gen_bool(state.config.error_simulation.timeout_probability) {
        return Err(AppError::Timeout);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, ErrorConfig, ResponseConfig, ServerConfig};

    fn test_config() -> Config {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            response: ResponseConfig {
                log_file: "test.log".to_string(),
                stream_delay_min_ms: 10,
                stream_delay_max_ms: 50,
            },
            error_simulation: ErrorConfig {
                enabled: false,
                rate_limit_probability: 0.0,
                timeout_probability: 0.0,
            },
        }
    }

    fn test_state() -> Arc<AppState> {
        Arc::new(AppState {
            config: test_config(),
            response_content: "Hello, world!".to_string(),
        })
    }

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("Hello world");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], "Hello ");
        assert_eq!(tokens[1], "world");
    }

    #[test]
    fn test_tokenize_multiple_words() {
        let tokens = tokenize("The quick brown fox");
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0], "The ");
        assert_eq!(tokens[1], "quick ");
        assert_eq!(tokens[2], "brown ");
        assert_eq!(tokens[3], "fox");
    }

    #[test]
    fn test_tokenize_empty() {
        let tokens = tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_tokenize_only_whitespace() {
        let tokens = tokenize("   ");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_check_error_simulation_disabled() {
        let state = test_state();
        assert!(check_error_simulation(&state).is_ok());
    }

    #[test]
    fn test_check_error_simulation_enabled_with_zero_probability() {
        let mut config = test_config();
        config.error_simulation.enabled = true;
        config.error_simulation.rate_limit_probability = 0.0;
        config.error_simulation.timeout_probability = 0.0;
        let state = Arc::new(AppState {
            config,
            response_content: "test".to_string(),
        });
        assert!(check_error_simulation(&state).is_ok());
    }
}
