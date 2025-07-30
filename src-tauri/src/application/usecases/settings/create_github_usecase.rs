use std::sync::Arc;

use anyhow::Result;

use crate::domain::models::settings::SettingsGithub;
use crate::domain::repositories::settings_repository::SettingsRepository;
use crate::domain::repositories::GithubApiRepository;

pub struct CreateGithubUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
    github_api_repository: Arc<dyn GithubApiRepository>,
}

impl CreateGithubUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>, github_api_repository: Arc<dyn GithubApiRepository>) -> Self {
        Self {
            settings_repository,
            github_api_repository,
        }
    }

    pub async fn execute(&self, personal_access_token: String) -> Result<SettingsGithub> {
        let user = self.github_api_repository.get_user(personal_access_token.clone()).await?;
        self.settings_repository
            .create_github(user.username, personal_access_token)
            .await
    }
}
