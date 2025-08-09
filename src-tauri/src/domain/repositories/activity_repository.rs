use anyhow::Result;

use crate::domain::models::activity::Activity;

#[async_trait::async_trait]
pub trait ActivityRepository: Send + Sync {
    async fn save(&self, activity: Activity) -> Result<()>;
}
