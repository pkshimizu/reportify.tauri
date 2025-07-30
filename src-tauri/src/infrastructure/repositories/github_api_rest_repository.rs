use anyhow::Result;
use reqwest;
use serde::Deserialize;

use crate::domain::models::github::GitHubUser;
use crate::domain::repositories::GithubApiRepository;

#[derive(Deserialize)]
struct GitHubApiUser {
    id: i32,
    login: String,
    avatar_url: String,
}

pub struct GithubApiRestRepository {
    client: reqwest::Client,
}

impl GithubApiRestRepository {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl GithubApiRepository for GithubApiRestRepository {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", personal_access_token))
            .header("User-Agent", "reportify")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let github_user: GitHubApiUser = response.json().await?;

        Ok(GitHubUser {
            id: github_user.id,
            username: github_user.login,
            avatar_url: Some(github_user.avatar_url),
        })
    }
}
