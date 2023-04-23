use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to fetch the rss feed.")]
    FetchFeedError(#[from] reqwest::Error),

    #[error("Failed to parse source as feed. Details: {0}")]
    ParseFeedError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let error = self.to_string();

        match self {
            AppError::FetchFeedError(_) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": error })))
            }
            AppError::ParseFeedError(_) => {
                (StatusCode::BAD_REQUEST, Json(json!({ "error": error })))
            }
        }
        .into_response()
    }
}
