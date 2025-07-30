use anyhow::Result;

use crate::domain::models::github::GitHubUser;
use crate::domain::repositories::GithubApiRepository;

pub struct GithubApiRestRepository {}

impl GithubApiRestRepository {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl GithubApiRepository for GithubApiRestRepository {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser> {
        Ok(GitHubUser {
            id: 1,
            username: "test".to_string(),
            avatar_url: Some("https://github.com/test.png".to_string()),
        })
    }
}
