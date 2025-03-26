mod m20240321_172635_create_store_table;
mod m20240322_124354_create_password_table;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240321_172635_create_store_table::Migration),
            Box::new(m20240322_124354_create_password_table::Migration),
        ]
    }
}
