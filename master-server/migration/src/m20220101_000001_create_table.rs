use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Participants::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Participants::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Participants::CampusCard)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Participants::NickName).string().not_null())
                    .col(
                        ColumnDef::new(Participants::DateRegistered)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(Participants::LastTransaction).timestamp())
                    .col(
                        ColumnDef::new(Participants::TapeLeftCM)
                            .float()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TransactionLog::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TransactionLog::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TransactionLog::MachineName).string())
                    .col(ColumnDef::new(TransactionLog::ParticipantID).integer())
                    .col(
                        ColumnDef::new(TransactionLog::TapeDeductedCM)
                            .float()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(TransactionLog::Timestamp)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_TransactionLog_Participants")
                            .from(TransactionLog::Table, TransactionLog::ParticipantID)
                            .to(Participants::Table, Participants::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(TransactionLog::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Participants::Table).to_owned())
            .await?;


        Ok(())
    }
}

#[derive(DeriveIden)]
enum Participants {
    Table,
    Id,
    CampusCard,
    NickName,
    DateRegistered,
    TapeLeftCM,
    LastTransaction,
}

#[derive(DeriveIden)]
enum TransactionLog {
    Table,
    Id,
    ParticipantID,
    MachineName,
    TapeDeductedCM,
    Timestamp,
}
