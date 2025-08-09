use anyhow::Result;
use reqwest;
use serde::Deserialize;

use crate::domain::models::github::{GitHubEvent, GitHubEventActor, GitHubEventRepo, GitHubUser};
use crate::domain::repositories::GithubApiRepository;

#[derive(Deserialize)]
struct GitHubApiUser {
    id: i32,
    login: String,
    avatar_url: String,
}

#[derive(Deserialize)]
struct GitHubApiEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    actor: GitHubApiEventActor,
    repo: GitHubApiEventRepo,
    payload: serde_json::Value,
    public: bool,
    created_at: String,
}

#[derive(Deserialize)]
struct GitHubApiEventActor {
    id: i32,
    login: String,
    avatar_url: String,
}

#[derive(Deserialize)]
struct GitHubApiEventRepo {
    id: i32,
    name: String,
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
            .header("Authorization", format!("token {personal_access_token}"))
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

    async fn get_events(
        &self,
        username: String,
        personal_access_token: String,
    ) -> Result<Vec<GitHubEvent>> {
        let url = format!("https://api.github.com/users/{username}/events");
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {personal_access_token}"))
            .header("User-Agent", "reportify")
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let github_events: Vec<GitHubApiEvent> = response.json().await?;

        let events = github_events
            .into_iter()
            .map(|event| GitHubEvent {
                id: event.id,
                event_type: event.event_type,
                actor: GitHubEventActor {
                    id: event.actor.id,
                    login: event.actor.login,
                    avatar_url: event.actor.avatar_url,
                },
                repo: GitHubEventRepo {
                    id: event.repo.id,
                    name: event.repo.name,
                },
                payload: event.payload,
                public: event.public,
                created_at: chrono::DateTime::parse_from_rfc3339(&event.created_at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now()),
            })
            .collect();

        Ok(events)
    }
}
