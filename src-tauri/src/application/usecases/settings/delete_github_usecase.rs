use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::settings_repository::SettingsRepository;

pub struct DeleteGithubUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl DeleteGithubUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self, id: i32) -> Result<()> {
        self.settings_repository.delete_github(id).await
    }
}
