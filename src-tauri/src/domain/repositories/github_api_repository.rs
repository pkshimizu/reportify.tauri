use anyhow::Result;

use crate::domain::models::github::GitHubUser;

#[async_trait::async_trait]
pub trait GithubApiRepository: Send + Sync {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser>;
}