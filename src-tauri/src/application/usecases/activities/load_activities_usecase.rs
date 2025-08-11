use anyhow::Result;
use std::sync::Arc;

use crate::domain::models::activity::Activity;
use crate::domain::repositories::activity_repository::ActivityRepository;

pub struct LoadActivitiesUseCase {
    activity_repository: Arc<dyn ActivityRepository>,
}

impl LoadActivitiesUseCase {
    pub fn new(activity_repository: Arc<dyn ActivityRepository>) -> Self {
        Self {
            activity_repository,
        }
    }

    pub async fn execute(&self, year: i32, month: u32) -> Result<Vec<Activity>> {
        self.activity_repository.find_activities(year, month).await
    }
}