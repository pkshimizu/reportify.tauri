use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SettingsGithub::Table)
                    .add_column(
                        ColumnDef::new(SettingsGithub::GithubId)
                            .integer()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SettingsGithub::Table)
                    .add_column(ColumnDef::new(SettingsGithub::AvatarUrl).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SettingsGithub::Table)
                    .drop_column(SettingsGithub::AvatarUrl)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SettingsGithub::Table)
                    .drop_column(SettingsGithub::GithubId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum SettingsGithub {
    Table,
    GithubId,
    AvatarUrl,
}
