use anyhow::Result;

use crate::domain::entities::{Settings, Theme};

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn get_settings(&self) -> Result<Settings>;
    async fn update_theme(&self, theme: Theme) -> Result<()>;
}
