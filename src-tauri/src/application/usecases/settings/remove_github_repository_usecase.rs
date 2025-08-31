use std::sync::Arc;

use anyhow::Result;

use crate::domain::repositories::SettingsRepository;

pub struct RemoveGitHubRepositoryUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl RemoveGitHubRepositoryUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self, github_repository_id: i32) -> Result<()> {
        self.settings_repository
            .remove_github_repository(github_repository_id)
            .await
    }
}
