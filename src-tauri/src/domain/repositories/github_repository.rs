use anyhow::Result;

use crate::domain::models::github::GitHubEvent;

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait GitHubRepository: Send + Sync {
    async fn save_event(&self, event: GitHubEvent) -> Result<()>;
}
