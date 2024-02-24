use super::error::{ok, ok_status};
use crate::api::error::{err, ErrorResponse};
use crate::db::entities::participants::{
    self, ActiveModel as ActiveParticipant, Entity as ParticipantTable, Model as Participant,
};
use crate::internal_error;
use crate::names::new_name;
use log::error;
use sea_orm::IntoSimpleExpr;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, Set,
};
use serde::Deserialize;
use warp::http::StatusCode;
use warp::reply;

fn now() -> sea_orm::prelude::ChronoDateTime {
    chrono::Local::now().naive_local()
}

async fn create_unused_name(db: &DatabaseConnection) -> Result<String, DbErr> {
    loop {
        let name = new_name();
        if ParticipantTable::find()
            .filter(participants::Column::NickName.like(&name))
            .one(db)
            .await?
            .is_none()
        {
            return Ok(name);
        }
    }
}

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
    if internal_error!(
        ParticipantTable::find()
            .filter(participants::Column::CampusCard.like(&campus_card))
            .one(&db)
            .await
    )
    .is_some()
    {
        return err(
            StatusCode::CONFLICT,
            "The campus card has already been registered",
        );
    }

    let name = internal_error!(create_unused_name(&db).await);

    let new_participant = ActiveParticipant {
        campus_card: Set(campus_card),
        nick_name: Set(name),
        ..Default::default()
    };

    new_participant
        .insert(&db)
        .await
        .map(|participant| ok_status(StatusCode::ACCEPTED, &participant))
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SetParams {
    tape_cm: f32,
}

pub async fn set_tape(
    campus_card: String,
    SetParams { tape_cm }: SetParams,
    db: DatabaseConnection,
) -> impl warp::Reply {
    let mut participant: ActiveParticipant = match find_by_campus_card(campus_card, &db).await {
        Ok(val) => val,
        Err(ex) => return ex,
    }
    .into();

    participant.tape_left_cm = Set(tape_cm);
    participant.last_transaction = Set(Some(now()));

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

pub async fn add_tape(
    campus_card: String,
    SetParams { tape_cm }: SetParams,
    db: DatabaseConnection,
) -> impl warp::Reply {
    let participant = match find_by_campus_card(campus_card, &db).await {
        Ok(val) => val,
        Err(ex) => return ex,
    };

    let old_tape_cm = participant.tape_left_cm;

    let mut participant: ActiveParticipant = participant.into();
    participant.tape_left_cm = Set(old_tape_cm + tape_cm);
    participant.last_transaction = Set(Some(now()));

    if let Err(ex) = participant.update(&db).await {
        error!("{ex}");
        err(StatusCode::INTERNAL_SERVER_ERROR, "Internal error")
    } else {
        ok_status(
            StatusCode::ACCEPTED,
            &super::types::TapeLeft {
                tape_left_cm: old_tape_cm + tape_cm,
            },
        )
    }
}

pub async fn add_all(SetParams { tape_cm }: SetParams, db: DatabaseConnection) -> impl warp::Reply {
    internal_error!(
        ParticipantTable::update_many()
            .col_expr(
                participants::Column::TapeLeftCm,
                Expr::add(
                    Expr::col(participants::Column::TapeLeftCm),
                    Expr::val(tape_cm),
                ),
            )
            .exec(&db)
            .await
    );

    super::error::ok_status(StatusCode::NO_CONTENT, &())
}

pub async fn set_all(SetParams { tape_cm }: SetParams, db: DatabaseConnection) -> impl warp::Reply {
    internal_error!(
        ParticipantTable::update_many()
            .col_expr(
                participants::Column::TapeLeftCm,
                Expr::val(tape_cm).into_simple_expr()
            )
            .exec(&db)
            .await
    );

    super::error::ok_status(StatusCode::NO_CONTENT, &())
}
