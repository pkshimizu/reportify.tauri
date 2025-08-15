use anyhow::Result;

use crate::domain::models::github::{GitHubEvent, GitHubUser};

#[async_trait::async_trait]
pub trait GithubApiRepository: Send + Sync {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser>;
    async fn get_events(
        &self,
        username: String,
        personal_access_token: String,
        latest_event_id: Option<String>,
    ) -> Result<Vec<GitHubEvent>>;
}
