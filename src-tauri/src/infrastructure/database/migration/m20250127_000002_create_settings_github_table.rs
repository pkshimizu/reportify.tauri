use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(
                        ColumnDef::new(SettingsGithub::PersonalAccessToken)
                            .string()
                            .not_null(),
                    )
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
            .await
    }
}

#[derive(Iden)]
enum SettingsGithub {
    Table,
    Id,
    PersonalAccessToken,
    CreatedAt,
    UpdatedAt,
}