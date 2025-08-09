use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    domain::{models::github::GitHubEvent, repositories::GitHubRepository},
    infrastructure::database::entities::{github_event, GitHubEventEntity},
};

pub struct GitHubDbRepository {
    db_connection: DatabaseConnection,
}

impl GitHubDbRepository {
    #[allow(dead_code)]
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl GitHubRepository for GitHubDbRepository {
    async fn save_event(&self, event: GitHubEvent) -> Result<bool> {
        let existing_event = GitHubEventEntity::find()
            .filter(github_event::Column::EventId.eq(event.id.clone()))
            .one(&self.db_connection)
            .await?;

        if existing_event.is_some() {
            return Ok(false);
        }

        let now = Utc::now();

        // Serialize payload to JSON string
        let payload_json = serde_json::to_string(&event.payload)?;

        let active_model = github_event::ActiveModel {
            id: sea_orm::NotSet,
            event_id: Set(event.id),
            event_type: Set(event.event_type),
            actor_id: Set(event.actor.id),
            actor_login: Set(event.actor.login),
            actor_avatar_url: Set(event.actor.avatar_url),
            repo_id: Set(event.repo.id),
            repo_name: Set(event.repo.name),
            payload: Set(payload_json),
            public: Set(event.public),
            created_at: Set(event.created_at),
            updated_at: Set(now),
        };

        active_model.insert(&self.db_connection).await?;

        Ok(true)
    }
}
