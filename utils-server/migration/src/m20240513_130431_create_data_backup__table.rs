use crate::sea_orm::EnumIter;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::Iterable;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        //     manager
        //      .create_type(
        //     Type::create()
        //         .as_enum(LastAccessMethod)
        //         .values(LastAccessMethodVariants::iter())
        //         .to_owned(),
        // ).await;

        manager
            .create_table(
                Table::create()
                    .table(Backup::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Backup::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Backup::UserId).uuid().not_null())
                    .col(ColumnDef::new(Backup::Key).string().not_null())
                    .col(ColumnDef::new(Backup::Value).string().not_null())
                    .col(
                        ColumnDef::new(Backup::DateAdded)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Backup::LastBackup)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Backup::LastAccessMethod).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(Backup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Backup {
    Table,
    Id,
    UserId,
    Key,
    Value,
    DateAdded,
    LastBackup,
    LastAccess,
    LastAccessMethod,
}

#[derive(Iden, EnumIter)]
pub enum LastAccessMethod {
    #[iden = "web"]
    Web,
    #[iden = "cli"]
    Cli,
}

type LastAccessMethodVariants = LastAccessMethod;
