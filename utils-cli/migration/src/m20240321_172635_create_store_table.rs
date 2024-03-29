use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Store::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Store::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Store::Key).string().not_null())
                    .col(ColumnDef::new(Store::Value).string().not_null())
                    .col(ColumnDef::new(Store::DateAdded).string().not_null())
                    .col(ColumnDef::new(Store::LastUpdated).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Store::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Store {
    Table,
    Id,
    Key,
    Value,
    DateAdded,
    LastUpdated,
}
