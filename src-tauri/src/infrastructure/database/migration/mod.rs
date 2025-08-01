use sea_orm_migration::prelude::*;

pub use sea_orm_migration::MigratorTrait;

mod m20250130_000001_create_all_tables;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250130_000001_create_all_tables::Migration)]
    }
}
