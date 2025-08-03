use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create github_events table
        manager
            .create_table(
                Table::create()
                    .table(GitHubEvents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GitHubEvents::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(GitHubEvents::EventId)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(GitHubEvents::EventType).string().not_null())
                    .col(ColumnDef::new(GitHubEvents::ActorId).integer().not_null())
                    .col(ColumnDef::new(GitHubEvents::ActorLogin).string().not_null())
                    .col(
                        ColumnDef::new(GitHubEvents::ActorAvatarUrl)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GitHubEvents::RepoId).integer().not_null())
                    .col(ColumnDef::new(GitHubEvents::RepoName).string().not_null())
                    .col(ColumnDef::new(GitHubEvents::Payload).text().not_null())
                    .col(ColumnDef::new(GitHubEvents::Public).boolean().not_null())
                    .col(
                        ColumnDef::new(GitHubEvents::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(GitHubEvents::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create indexes for github_events table
        manager
            .create_index(
                Index::create()
                    .name("idx_github_events_event_type")
                    .table(GitHubEvents::Table)
                    .col(GitHubEvents::EventType)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_github_events_created_at")
                    .table(GitHubEvents::Table)
                    .col(GitHubEvents::CreatedAt)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_github_events_actor_id")
                    .table(GitHubEvents::Table)
                    .col(GitHubEvents::ActorId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GitHubEvents::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum GitHubEvents {
    Table,
    Id,
    EventId,
    EventType,
    ActorId,
    ActorLogin,
    ActorAvatarUrl,
    RepoId,
    RepoName,
    Payload,
    Public,
    CreatedAt,
    UpdatedAt,
}
