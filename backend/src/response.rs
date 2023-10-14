use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
struct BodyResponse<T> {
    data: T,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub enum CustomResponse<T> {
    Success(T),
    Error(String),
}

impl<T: Serialize> IntoResponse for CustomResponse<T> {
    fn into_response(self) -> Response {
        match self {
            CustomResponse::Success(body) => {
                let body_response = BodyResponse { data: body };
                (StatusCode::OK, Json(body_response)).into_response()
            }
            CustomResponse::Error(error) => {
                let error_response = ErrorResponse { error };
                (StatusCode::FORBIDDEN, Json(error_response)).into_response()
            }
        }
    }
}
