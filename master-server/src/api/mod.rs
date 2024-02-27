use self::{
    cards::{
        add_all, add_tape, list_campus_cards, lookup_campus_card, regenerate_name,
        register_campus_card, set_all, set_tape,
    },
    error::Error,
};
use crate::env_config::ENV_CONFIG;
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    routing::{get, post, put},
    Router,
};

mod cards;
pub mod error;
mod types;

struct Auth;

#[axum::async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Sync + Send,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get("password") {
            Some(auth) if auth.to_str().unwrap_or_default() == ENV_CONFIG.password => Ok(Self),
            Some(_) | None => Err(Error::NotAuthenticated),
        }
    }
}

async fn login(_: Auth) -> (StatusCode, ()) {
    (StatusCode::ACCEPTED, ())
}

pub fn router() -> Router<sea_orm::DatabaseConnection> {
    Router::new()
        .route("/login", get(login))
        .route("/campus_card", get(list_campus_cards))
        .route("/campus_card/set", post(set_all))
        .route("/campus_card/add", post(add_all))
        .route(
            "/campus_card/:campus_card",
            put(register_campus_card).get(lookup_campus_card),
        )
        .route("/campus_card/:campus_card/add", post(add_tape))
        .route("/campus_card/:campus_card/set", post(set_tape))
        .route("/campus_card/:campus_card/new_name", post(regenerate_name))
}
