use crate::domain::entities::Theme;
use anyhow::Result;

#[async_trait::async_trait]
pub trait ThemeRepository: Send + Sync {
    async fn get_theme(&self) -> Result<Theme>;
    async fn update_theme(&self, theme: Theme) -> Result<()>;
}