use anyhow::Result;

use crate::domain::models::activity::Activity;

#[async_trait::async_trait]
pub trait ActivityRepository: Send + Sync {
    async fn save(&self, activity: Activity) -> Result<()>;
    async fn find_activities(&self, year: i32, month: u32) -> Result<Vec<Activity>>;
}
