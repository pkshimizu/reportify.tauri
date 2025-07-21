use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::{Settings, Theme},
    repositories::SettingsRepository,
};

pub struct SettingsUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl SettingsUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn get_settings(&self) -> Result<Settings> {
        self.settings_repository.get_settings().await
    }

    pub async fn update_theme(&self, theme: Theme) -> Result<()> {
        self.settings_repository.update_theme(theme).await
    }
}
