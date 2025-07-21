use std::sync::Arc;

use anyhow::Result;

use crate::domain::{
    entities::Theme,
    repositories::ThemeRepository,
};

pub struct ThemeUseCase {
    theme_repository: Arc<dyn ThemeRepository>,
}

impl ThemeUseCase {
    pub fn new(theme_repository: Arc<dyn ThemeRepository>) -> Self {
        Self { theme_repository }
    }

    pub async fn get_theme(&self) -> Result<Theme> {
        self.theme_repository.get_theme().await
    }

    pub async fn update_theme(&self, theme: Theme) -> Result<()> {
        self.theme_repository.update_theme(theme).await
    }
}
