use serde::{Deserialize, Serialize};
use warp::{
    http::StatusCode,
    reply::{self},
};

#[macro_export]
macro_rules! internal_error {
    ($item:expr) => {
        match $item {
            Ok(val) => val,
            Err(ex) => {
                log::error!("{ex}");
                return $crate::api::error::err(
                    warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal error",
                );
            }
        }
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ErrorResponse {
    pub error_message: String,
}

pub fn ok<T>(payload: &T) -> reply::WithStatus<reply::Json>
where
    T: serde::Serialize,
{
    reply::with_status(reply::json(payload), StatusCode::OK)
}

pub fn ok_status<T>(status_code: StatusCode, payload: &T) -> reply::WithStatus<reply::Json>
where
    T: serde::Serialize,
{
    reply::with_status(reply::json(payload), status_code)
}

pub fn err(status_code: StatusCode, msg: impl ToString) -> reply::WithStatus<reply::Json> {
    reply::with_status(
        reply::json(&ErrorResponse {
            error_message: msg.to_string(),
        }),
        status_code,
    )
}

pub async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(warp::reply::json(&format!("{:?}", err)))
}