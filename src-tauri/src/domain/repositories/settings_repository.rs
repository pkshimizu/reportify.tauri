use anyhow::Result;

use crate::domain::models::settings::{Settings, SettingsGithub};
use crate::domain::models::Theme;

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn load_settings(&self) -> Result<Settings>;
    async fn save_theme(&self, theme: Theme) -> Result<()>;

    // GitHub settings methods
    async fn load_githubs(&self) -> Result<Vec<SettingsGithub>>;
    async fn create_github(&self, personal_access_token: String) -> Result<SettingsGithub>;
    async fn delete_github(&self, id: i32) -> Result<()>;
}
