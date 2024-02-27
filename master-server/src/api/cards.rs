use crate::db::entities::participants::{
    self, ActiveModel as ActiveParticipant, Entity as ParticipantTable, Model as Participant,
};
use crate::names::new_name;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, Set,
};
use sea_orm::{IntoActiveModel, IntoSimpleExpr};
use serde::Deserialize;

use super::error::Error;
use super::types::{Participants, TapeLeft};
use super::Auth;

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
) -> Result<Participant, Error> {
    ParticipantTable::find()
        .filter(participants::Column::CampusCard.like(&campus_card))
        .one(db)
        .await?
        .ok_or_else(|| Error::NotFound {
            resource: format!("Campus card id {campus_card}"),
        })
}

pub async fn register_campus_card(
    Path(campus_card): Path<String>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<Json<Participant>, Error> {
    // Check if existing
    if ParticipantTable::find()
        .filter(participants::Column::CampusCard.like(&campus_card))
        .one(db)
        .await?
        .is_some()
    {
        return Err(Error::Conflict {
            resource: format!("Campus card id {campus_card}"),
        });
    }

    let name = create_unused_name(db).await?;

    let new_participant = ActiveParticipant {
        campus_card: Set(campus_card),
        nick_name: Set(name),
        ..Default::default()
    };

    new_participant
        .insert(db)
        .await
        .map(|participant| Ok(Json(participant)))?
}

pub async fn lookup_campus_card(
    Path(campus_card): Path<String>,
    State(ref db): State<DatabaseConnection>,
) -> Result<Json<Participant>, Error> {
    Ok(Json(find_by_campus_card(campus_card, db).await?))
}

pub async fn list_campus_cards(
    State(ref db): State<DatabaseConnection>,
) -> Result<Json<Participants>, Error> {
    Ok(Json(super::types::Participants {
        participants: ParticipantTable::find().all(db).await?,
    }))
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct SetParams {
    tape_cm: f32,
}

pub async fn set_tape(
    Path(campus_card): Path<String>,
    Query(SetParams { tape_cm }): Query<SetParams>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<Json<TapeLeft>, Error> {
    let mut participant = find_by_campus_card(campus_card, db)
        .await?
        .into_active_model();

    participant.tape_left_cm = Set(tape_cm);
    participant.last_transaction = Set(Some(now()));

    participant.update(db).await?;

    Ok(Json(super::types::TapeLeft {
        tape_left_cm: tape_cm,
    }))
}

pub async fn add_tape(
    Path(campus_card): Path<String>,
    Query(SetParams { tape_cm }): Query<SetParams>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<Json<TapeLeft>, Error> {
    let participant = find_by_campus_card(campus_card, db).await?;

    let old_tape_cm = participant.tape_left_cm;

    let mut participant: ActiveParticipant = participant.into();
    participant.tape_left_cm = Set(old_tape_cm + tape_cm);
    participant.last_transaction = Set(Some(now()));

    participant.update(db).await?;
    Ok(Json(super::types::TapeLeft {
        tape_left_cm: old_tape_cm + tape_cm,
    }))
}

pub async fn add_all(
    Query(SetParams { tape_cm }): Query<SetParams>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<StatusCode, Error> {
    ParticipantTable::update_many()
        .col_expr(
            participants::Column::TapeLeftCm,
            Expr::add(
                Expr::col(participants::Column::TapeLeftCm),
                Expr::val(tape_cm),
            ),
        )
        .exec(db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn set_all(
    Query(SetParams { tape_cm }): Query<SetParams>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<StatusCode, Error> {
    ParticipantTable::update_many()
        .col_expr(
            participants::Column::TapeLeftCm,
            Expr::val(tape_cm).into_simple_expr(),
        )
        .exec(db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn regenerate_name(
    Path(campus_card): Path<String>,
    State(ref db): State<DatabaseConnection>,
    _: Auth,
) -> Result<Json<Participant>, Error> {
    let campus_card = find_by_campus_card(campus_card, db).await?;

    let mut new_campus_card = campus_card.clone().into_active_model();

    loop {
        let new_name = create_unused_name(db).await?;
        if new_name != campus_card.nick_name {
            new_campus_card.nick_name = Set(new_name);
            return Ok(Json(new_campus_card.update(db).await?));
        }
    }
}
