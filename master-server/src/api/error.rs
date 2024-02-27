use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use sea_orm::DbErr;
use serde_json::json;

pub enum Error {
    InternalServerError,
    NotFound { resource: String },
    NotAuthenticated,
    Conflict { resource: String },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error_message": "Internal server error"})),
            )
                .into_response(),
            Error::NotFound { resource } => (
                StatusCode::NOT_FOUND,
                Json(json!({"error_message": format!("{resource} not found")})),
            )
                .into_response(),
            Error::NotAuthenticated => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error_message": "You are not authorized to use this route"})),
            )
                .into_response(),
            Error::Conflict { resource } => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error_message": format!("{resource} already exists")})),
            )
                .into_response(),
        }
    }
}

impl From<DbErr> for Error {
    fn from(_value: DbErr) -> Self {
        Error::InternalServerError
    }
}

pub async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(warp::reply::json(&format!("{:?}", err)))
}