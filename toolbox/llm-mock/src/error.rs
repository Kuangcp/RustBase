use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    RateLimit,
    InternalError,
    Timeout,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            AppError::RateLimit => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limit_error",
                "Rate limit exceeded",
            ),
            AppError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "server_error",
                "Internal server error",
            ),
            AppError::Timeout => (
                StatusCode::GATEWAY_TIMEOUT,
                "timeout_error",
                "Request timed out",
            ),
        };

        let body = json!({
            "error": {
                "message": message,
                "type": error_type,
                "param": null,
                "code": error_type
            }
        });

        (status, axum::Json(body)).into_response()
    }
}
