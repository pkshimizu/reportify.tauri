use std::sync::Arc;

use anyhow::Result;

use crate::domain::{models::Theme, repositories::SettingsRepository};

pub struct SaveThemeUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl SaveThemeUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self, theme: Theme) -> Result<()> {
        self.settings_repository.save_theme(theme).await
    }
}
