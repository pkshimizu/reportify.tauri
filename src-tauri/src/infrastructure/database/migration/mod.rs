use sea_orm_migration::prelude::*;

pub use sea_orm_migration::MigratorTrait;

mod m20240721_000001_create_settings_table;
mod m20250127_000001_create_activities_table;
mod m20250127_000002_create_settings_github_table;
mod m20250130_000001_add_original_url_to_activities;
mod m20250130_000002_add_username_to_settings_github;
mod m20250130_000003_add_github_fields_to_settings_github;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240721_000001_create_settings_table::Migration),
            Box::new(m20250127_000001_create_activities_table::Migration),
            Box::new(m20250127_000002_create_settings_github_table::Migration),
            Box::new(m20250130_000001_add_original_url_to_activities::Migration),
            Box::new(m20250130_000002_add_username_to_settings_github::Migration),
            Box::new(m20250130_000003_add_github_fields_to_settings_github::Migration),
        ]
    }
}
