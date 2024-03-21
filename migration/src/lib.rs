pub use sea_orm_migration::prelude::*;

mod m20240321_172635_create_store_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240321_172635_create_store_table::Migration),
        ]
    }
}
