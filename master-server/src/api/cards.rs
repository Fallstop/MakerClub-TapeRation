use crate::api::error::{err, ErrorResponse};
use crate::db::entities::participants::{
    self, ActiveModel as ActiveParticipant, Entity as ParticipantTable, Model as Participant,
};
use log::error;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::{DatabaseConnection, Set};
use warp::http::StatusCode;
use warp::reply;

use super::error::{ok, ok_status};

async fn find_by_campus_card(
    campus_card: String,
    db: &DatabaseConnection,
) -> Result<Participant, reply::WithStatus<reply::Json>> {
    match ParticipantTable::find()
        .filter(participants::Column::CampusCard.like(&campus_card))
        .one(db)
        .await
    {
        Ok(Some(participant)) => Ok(participant),
        Ok(None) => Err(reply::with_status(
            reply::json(&ErrorResponse {
                error_message: "Not found".into(),
            }),
            StatusCode::NOT_FOUND,
        )),
        Err(ex) => {
            error!("{ex}");
            Err(err(StatusCode::INTERNAL_SERVER_ERROR, "Internal error"))
        }
    }
}

pub async fn register_campus_card(campus_card: String, db: DatabaseConnection) -> impl warp::Reply {
    // Check if existing
    let new_participant = participants::ActiveModel {
        campus_card: Set(campus_card),
        ..Default::default()
    };

    new_participant
        .insert(&db)
        .await
        .map(|response| {
            ok_status(
                StatusCode::ACCEPTED,
                &super::types::TapeLeft {
                    tape_left_cm: response.tape_left_cm,
                },
            )
        })
        .unwrap_or_else(|ex| {
            error!("{}", ex);
            err(StatusCode::CONFLICT, "Participant already registered")
        })
}

pub async fn lookup_campus_card(campus_card: String, db: DatabaseConnection) -> impl warp::Reply {
    find_by_campus_card(campus_card, &db)
        .await
        .map(|res| ok(&res))
        .unwrap_or_else(|ex| ex)
}

pub async fn list_campus_cards(db: DatabaseConnection) -> impl warp::Reply {
    ParticipantTable::find()
        .all(&db)
        .await
        .map(|participants| ok(&super::types::Participants { participants }))
        .unwrap_or_else(|ex| {
            error!("{ex}");
            err(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
        })
}

pub async fn set_tape(
    campus_card: String,
    tape_cm: f32,
    db: DatabaseConnection,
) -> impl warp::Reply {
    let mut participant: ActiveParticipant = match find_by_campus_card(campus_card, &db).await {
        Ok(val) => val,
        Err(ex) => return ex,
    }
    .into();

    participant.tape_left_cm = Set(tape_cm);
    if let Err(ex) = participant.update(&db).await {
        error!("{ex}");
        err(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
    } else {
        ok_status(
            StatusCode::ACCEPTED,
            &super::types::TapeLeft {
                tape_left_cm: tape_cm,
            },
        )
    }
}
