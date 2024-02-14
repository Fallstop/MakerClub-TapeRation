use std::convert::Infallible;
use log::error;
use crate::api::error::ErrorResponse;
use crate::db::entities::participants;

use sea_orm::{DatabaseConnection, IntoActiveModel, Set};
use sea_orm::ActiveModelTrait;
use warp::reply;
use warp::http::StatusCode;

pub async fn register_campus_card(campus_card: String, db: DatabaseConnection) -> Result<impl warp::Reply, Infallible> {
    // Check if existing
    let new_participant = participants::ActiveModel {
        campus_card: Set(campus_card),
        ..Default::default()
    };

    match new_participant.insert(&db).await {
        Ok(response) => {
            Ok(
                reply::with_status(
                    reply::json(&response.tape_left_cm),
                    StatusCode::ACCEPTED
                )
            )
        }
        Err(err) => {
            error!("{}", err);
            Ok(
                reply::with_status(reply::json(&ErrorResponse {
                    error_message: String::from("Participant already registered")
                }), StatusCode::CONFLICT)
            )
        }
    }

}

pub async fn lookup_campus_card(campus_card: String, db: DatabaseConnection) -> Result<impl warp::Reply, Infallible> {
    Ok("Okay")
}

pub async fn list_campus_cards(db: DatabaseConnection) -> Result<impl warp::Reply, Infallible> {
    Ok("Okay")
}