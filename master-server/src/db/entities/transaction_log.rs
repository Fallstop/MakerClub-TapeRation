//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde :: Serialize, serde :: Deserialize)]
#[sea_orm(table_name = "transaction_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub machine_name: Option<String>,
    pub participant_id: Option<i32>,
    #[sea_orm(column_type = "Float")]
    pub tape_deducted_cm: f32,
    pub timestamp: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::participants::Entity",
        from = "Column::ParticipantId",
        to = "super::participants::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Participants,
}

impl Related<super::participants::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participants.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
