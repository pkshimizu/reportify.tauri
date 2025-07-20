use crate::domain::{entities::Theme, repositories::ThemeRepository};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct InMemoryThemeRepository {
    current_theme: Mutex<Theme>,
}

impl InMemoryThemeRepository {
    pub fn new() -> Self {
        Self {
            current_theme: Mutex::new(Theme::default()),
        }
    }

    pub fn new_with_theme(theme: Theme) -> Self {
        Self {
            current_theme: Mutex::new(theme),
        }
    }
}

#[async_trait]
impl ThemeRepository for InMemoryThemeRepository {
    async fn get_theme(&self) -> Result<Theme> {
        let theme = self
            .current_theme
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?
            .clone();
        Ok(theme)
    }

    async fn update_theme(&self, theme: Theme) -> Result<()> {
        let mut current_theme = self
            .current_theme
            .lock()
            .map_err(|e| anyhow::anyhow!("Failed to acquire lock: {}", e))?;
        *current_theme = theme;
        Ok(())
    }
}