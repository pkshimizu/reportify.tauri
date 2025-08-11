use anyhow::Result;
use async_trait::async_trait;
use chrono::{DateTime, Months, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

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
    async fn find_activities(&self, year: i32, month: u32) -> Result<Vec<Activity>> {
        let start_date: DateTime<Utc> = DateTime::from_naive_utc_and_offset(
            chrono::NaiveDate::from_ymd_opt(year, month, 1)
                .ok_or_else(|| anyhow::anyhow!("Invalid date: {}-{}", year, month))?
                .and_hms_opt(0, 0, 0)
                .unwrap(),
            Utc,
        );
        let end_date = start_date + Months::new(1);

        let models = activity::Entity::find()
            .filter(activity::Column::CreatedAt.gte(start_date))
            .filter(activity::Column::CreatedAt.lt(end_date))
            .all(&self.db_connection)
            .await?;

        let activities = models
            .into_iter()
            .map(|model| Activity {
                service: model.service,
                activity_type: model.activity_type,
                summary: model.summary,
                detail: model.detail,
                original_url: model.original_url,
                created_at: model.created_at,
            })
            .collect();

        Ok(activities)
    }
}
