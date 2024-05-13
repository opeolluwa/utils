use crate::sea_orm::EnumIter;
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
                    .table(UserInformation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInformation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(UserInformation::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(UserInformation::Username).string())
                    .col(
                        ColumnDef::new(UserInformation::Fullname)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(UserInformation::LastBackup).timestamp_with_time_zone())
                    .col(ColumnDef::new(UserInformation::LastLogin).timestamp_with_time_zone())
                    .col(
                        ColumnDef::new(UserInformation::SecurityQuestion)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(UserInformation::SecurityQuestionAnswer)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInformation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum UserInformation {
    Table,
    Id,
    Username,
    Fullname,
    Email,
    LastBackup,
    LastLogin,
    SecurityQuestion,
    SecurityQuestionAnswer,
}
