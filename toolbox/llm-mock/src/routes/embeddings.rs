use axum::Json;
use crate::error::AppError;
use crate::models::openai_types::{EmbeddingData, EmbeddingResponse, Usage};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct EmbeddingRequest {
    pub input: String,
    pub model: Option<String>,
}

pub async fn create_embedding(
    Json(payload): Json<EmbeddingRequest>,
) -> Result<Json<EmbeddingResponse>, AppError> {
    let embedding = vec![0.1; 1536];
    
    let response = EmbeddingResponse {
        object: "list".to_string(),
        data: vec![EmbeddingData {
            object: "embedding".to_string(),
            embedding,
            index: 0,
        }],
        model: payload.model.unwrap_or_else(|| "text-embedding-ada-002".to_string()),
        usage: Usage {
            prompt_tokens: 8,
            total_tokens: 8,
        },
    };
    
    Ok(Json(response))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::post;
    use axum::Router;
    use tower::ServiceExt;

    fn test_app() -> Router {
        Router::new().route("/v1/embeddings", post(create_embedding))
    }

    #[tokio::test]
    async fn test_embeddings_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_embeddings_returns_json() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let content_type = response.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));
    }

    #[tokio::test]
    async fn test_embeddings_response_structure() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let embedding_response: EmbeddingResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(embedding_response.object, "list");
        assert_eq!(embedding_response.data.len(), 1);
        assert_eq!(embedding_response.data[0].object, "embedding");
        assert_eq!(embedding_response.data[0].embedding.len(), 1536);
        assert_eq!(embedding_response.data[0].index, 0);
    }

    #[tokio::test]
    async fn test_embeddings_default_model() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let embedding_response: EmbeddingResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(embedding_response.model, "text-embedding-ada-002");
    }

    #[tokio::test]
    async fn test_embeddings_custom_model() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world", "model": "text-embedding-3-small"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let embedding_response: EmbeddingResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(embedding_response.model, "text-embedding-3-small");
    }

    #[tokio::test]
    async fn test_embeddings_usage() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/embeddings")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"input": "hello world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let embedding_response: EmbeddingResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(embedding_response.usage.prompt_tokens, 8);
        assert_eq!(embedding_response.usage.total_tokens, 8);
    }
}
