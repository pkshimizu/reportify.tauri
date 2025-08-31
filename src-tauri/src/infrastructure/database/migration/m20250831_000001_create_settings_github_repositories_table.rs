use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create settings_github_repositories table
        manager
            .create_table(
                Table::create()
                    .table(Alias::new("settings_github_repositories"))
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::GithubRepositoryId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::Name)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::FullName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::Description)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::Private)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::OwnerName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SettingsGithubRepositories::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Alias::new("settings_github_repositories"))
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
#[allow(dead_code)]
enum SettingsGithubRepositories {
    Table,
    Id,
    SettingsGithubId,
    GithubRepositoryId,
    Name,
    FullName,
    Description,
    Private,
    OwnerName,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
