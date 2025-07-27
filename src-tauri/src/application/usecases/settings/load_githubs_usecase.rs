use std::sync::Arc;

use anyhow::Result;

use crate::domain::models::settings::SettingsGithub;
use crate::domain::repositories::settings_repository::SettingsRepository;

pub struct LoadGithubsUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl LoadGithubsUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self) -> Result<Vec<SettingsGithub>> {
        self.settings_repository.load_githubs().await
    }
}
