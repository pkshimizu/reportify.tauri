use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
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
                    .col(
                        ColumnDef::new(Activities::Service)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Activities::ActivityType)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Activities::Summary)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Activities::Detail)
                            .text()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Activities::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // Create index on service for efficient filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_activities_service")
                    .table(Activities::Table)
                    .col(Activities::Service)
                    .to_owned(),
            )
            .await?;

        // Create index on activity_type for efficient filtering
        manager
            .create_index(
                Index::create()
                    .name("idx_activities_type")
                    .table(Activities::Table)
                    .col(Activities::ActivityType)
                    .to_owned(),
            )
            .await?;

        // Create index on created_at for efficient date range queries
        manager
            .create_index(
                Index::create()
                    .name("idx_activities_created_at")
                    .table(Activities::Table)
                    .col(Activities::CreatedAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activities::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Activities {
    Table,
    Id,
    Service,
    ActivityType,
    Summary,
    Detail,
    CreatedAt,
}