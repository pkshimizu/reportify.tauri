use anyhow::Result;

use crate::domain::models::github::GitHubRepository;
use crate::domain::models::settings::{Settings, SettingsGithub, SettingsGithubRepository};
use crate::domain::models::{GitHubUser, Theme};

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn load_settings(&self) -> Result<Settings>;
    async fn save_theme(&self, theme: Theme) -> Result<()>;

    // GitHub settings methods
    async fn load_githubs(&self) -> Result<Vec<SettingsGithub>>;
    async fn create_github(
        &self,
        user: GitHubUser,
        personal_access_token: String,
    ) -> Result<SettingsGithub>;
    async fn delete_github(&self, id: i32) -> Result<()>;
    async fn save_github_latest_event_id(
        &self,
        setting_github_id: i32,
        github_event_id: String,
    ) -> Result<()>;

    // GitHub repository settings methods
    async fn add_github_repository(
        &self,
        repository: GitHubRepository,
    ) -> Result<SettingsGithubRepository>;
    async fn remove_github_repository(&self, github_repository_id: i32) -> Result<()>;
}
