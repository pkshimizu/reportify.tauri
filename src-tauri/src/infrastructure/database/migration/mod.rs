use sea_orm_migration::prelude::*;

pub use sea_orm_migration::MigratorTrait;

mod m20240721_000001_create_settings_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240721_000001_create_settings_table::Migration)]
    }
}
