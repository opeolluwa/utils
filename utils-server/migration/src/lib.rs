pub use sea_orm_migration::prelude::*;

mod m20240513_124422_create_user_information_table;
mod m20240513_130431_create_data_backup__table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240513_124422_create_user_information_table::Migration),
            Box::new(m20240513_130431_create_data_backup__table::Migration),
          
        ]
    }
}
