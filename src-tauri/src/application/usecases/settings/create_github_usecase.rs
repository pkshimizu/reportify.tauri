use std::sync::Arc;

use anyhow::Result;

use crate::domain::models::settings::SettingsGithub;
use crate::domain::repositories::settings_repository::SettingsRepository;

pub struct CreateGithubUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl CreateGithubUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self, personal_access_token: String) -> Result<SettingsGithub> {
        self.settings_repository
            .create_github(personal_access_token)
            .await
    }
}
