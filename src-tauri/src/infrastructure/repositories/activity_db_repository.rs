use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::domain::models::activity::Activity;
use crate::domain::repositories::activity_repository::ActivityRepository;
use crate::infrastructure::database::entities::activity;

pub struct ActivityDbRepository {
    db_connection: DatabaseConnection,
}

impl ActivityDbRepository {
    #[allow(dead_code)]
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl ActivityRepository for ActivityDbRepository {
    async fn save(&self, activity_model: Activity) -> Result<()> {
        let created_at = activity_model.created_at;
        let created_at = if created_at.timestamp() == 0 {
            Utc::now()
        } else {
            created_at
        };

        let active_model = activity::ActiveModel {
            id: sea_orm::NotSet,
            service: Set(activity_model.service),
            activity_type: Set(activity_model.activity_type),
            summary: Set(activity_model.summary),
            detail: Set(activity_model.detail),
            original_url: Set(activity_model.original_url),
            created_at: Set(created_at),
        };

        active_model.insert(&self.db_connection).await?;
        Ok(())
    }
}
