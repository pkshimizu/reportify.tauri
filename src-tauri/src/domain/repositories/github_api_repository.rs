use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::domain::models::github::{
    GitHubEvent, GitHubEventsCollection, GitHubRepository, GitHubUser,
};

#[async_trait::async_trait]
pub trait GithubApiRepository: Send + Sync {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser>;
    async fn get_events(
        &self,
        username: String,
        personal_access_token: String,
        latest_event_id: Option<String>,
    ) -> Result<Vec<GitHubEvent>>;
    async fn get_repositories(
        &self,
        personal_access_token: String,
    ) -> Result<Vec<GitHubRepository>>;
    #[allow(dead_code)]
    async fn get_events_collection(
        &self,
        username: String,
        repository_name: String,
        personal_access_token: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<GitHubEventsCollection>;
}
