use std::sync::Arc;

use anyhow::Result;

use crate::domain::{models::Settings, repositories::SettingsRepository};

pub struct LoadSettingsUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
}

impl LoadSettingsUseCase {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }

    pub async fn execute(&self) -> Result<Settings> {
        self.settings_repository.load_settings().await
    }
}
