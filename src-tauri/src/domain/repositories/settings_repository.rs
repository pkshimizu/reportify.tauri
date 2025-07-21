use anyhow::Result;

use crate::domain::models::{Settings, Theme};

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn load_settings(&self) -> Result<Settings>;
    async fn save_theme(&self, theme: Theme) -> Result<()>;
}
