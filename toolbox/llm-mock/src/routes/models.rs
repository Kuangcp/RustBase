use axum::Json;
use crate::models::openai_types::{Model, ModelList};

pub async fn list_models() -> Json<ModelList> {
    let models = ModelList {
        data: vec![
            Model {
                id: "gpt-3.5-turbo".to_string(),
                object: "model".to_string(),
                owned_by: "mock".to_string(),
            },
            Model {
                id: "gpt-4".to_string(),
                object: "model".to_string(),
                owned_by: "mock".to_string(),
            },
        ],
    };
    Json(models)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::routing::get;
    use axum::Router;
    use tower::ServiceExt;

    fn test_app() -> Router {
        Router::new().route("/v1/models", get(list_models))
    }

    #[tokio::test]
    async fn test_models_endpoint_returns_200() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/models")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_models_endpoint_returns_json() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/models")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let content_type = response.headers().get("content-type").unwrap();
        assert!(content_type.to_str().unwrap().contains("application/json"));
    }

    #[tokio::test]
    async fn test_models_endpoint_contains_expected_models() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/models")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let models: ModelList = serde_json::from_slice(&body).unwrap();
        let model_ids: Vec<&str> = models.data.iter().map(|m| m.id.as_str()).collect();
        assert!(model_ids.contains(&"gpt-3.5-turbo"));
        assert!(model_ids.contains(&"gpt-4"));
    }

    #[tokio::test]
    async fn test_models_endpoint_model_fields() {
        let app = test_app();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/v1/models")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let models: ModelList = serde_json::from_slice(&body).unwrap();
        for model in &models.data {
            assert_eq!(model.object, "model");
            assert_eq!(model.owned_by, "mock");
        }
    }
}
