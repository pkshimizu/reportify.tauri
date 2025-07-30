use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create settings table
        manager
            .create_table(
                Table::create()
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Settings::Theme)
                            .string()
                            .not_null()
                            .default("light"),
                    )
                    .col(
                        ColumnDef::new(Settings::Language)
                            .string()
                            .not_null()
                            .default("en"),
                    )
                    .col(
                        ColumnDef::new(Settings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Settings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Insert default settings
        let insert = Query::insert()
            .into_table(Settings::Table)
            .columns([Settings::Id, Settings::Theme, Settings::Language])
            .values_panic([1.into(), "light".into(), "en".into()])
            .to_owned();

        manager.exec_stmt(insert).await?;

        // Create activities table
        manager
            .create_table(
                Table::create()
                    .table(Activities::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Activities::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Activities::Service).string().not_null())
                    .col(ColumnDef::new(Activities::ActivityType).string().not_null())
                    .col(ColumnDef::new(Activities::Summary).string().not_null())
                    .col(ColumnDef::new(Activities::Detail).text().not_null())
                    .col(ColumnDef::new(Activities::OriginalUrl).string().null())
                    .col(
                        ColumnDef::new(Activities::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for activities table
        manager
            .create_index(
                Index::create()
                    .name("idx_activities_service")
                    .table(Activities::Table)
                    .col(Activities::Service)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_activities_type")
                    .table(Activities::Table)
                    .col(Activities::ActivityType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_activities_created_at")
                    .table(Activities::Table)
                    .col(Activities::CreatedAt)
                    .to_owned(),
            )
            .await?;

        // Create settings_github table
        manager
            .create_table(
                Table::create()
                    .table(SettingsGithub::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SettingsGithub::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SettingsGithub::Username).string().not_null())
                    .col(
                        ColumnDef::new(SettingsGithub::GithubId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(SettingsGithub::PersonalAccessToken)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(SettingsGithub::AvatarUrl).string().null())
                    .col(
                        ColumnDef::new(SettingsGithub::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(SettingsGithub::UpdatedAt)
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
            .drop_table(Table::drop().table(SettingsGithub::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Activities::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
enum Settings {
    Table,
    Id,
    Theme,
    Language,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
enum Activities {
    Table,
    Id,
    Service,
    ActivityType,
    Summary,
    Detail,
    OriginalUrl,
    CreatedAt,
}

#[derive(Iden)]
enum SettingsGithub {
    Table,
    Id,
    Username,
    GithubId,
    PersonalAccessToken,
    AvatarUrl,
    CreatedAt,
    UpdatedAt,
}
