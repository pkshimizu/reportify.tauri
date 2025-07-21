use std::sync::{
    Arc,
    Mutex,
};

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;

use crate::{
    domain::{
        entities::Theme,
        repositories::ThemeRepository,
    },
    infrastructure::database::{
        models::*,
        schema::theme_settings,
    },
};

pub struct SqliteThemeRepository {
    db_connection: Arc<Mutex<SqliteConnection>>,
}

impl SqliteThemeRepository {
    pub fn new(db_connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl ThemeRepository for SqliteThemeRepository {
    async fn get_theme(&self) -> Result<Theme> {
        let conn = Arc::clone(&self.db_connection);
        let result = tokio::task::spawn_blocking(move || -> Result<Theme> {
            let mut connection = conn.lock().unwrap();

            let theme_setting: ThemeSetting = theme_settings::table
                .filter(theme_settings::id.eq(1))
                .first(&mut *connection)
                .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

            Theme::from_string(&theme_setting.theme_name)
                .map_err(|e| anyhow::anyhow!("Theme parsing error: {}", e))
        })
        .await??;

        Ok(result)
    }

    async fn update_theme(&self, theme: Theme) -> Result<()> {
        let conn = Arc::clone(&self.db_connection);
        let theme_str = theme.to_string();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut connection = conn.lock().unwrap();
            let now = Utc::now().naive_utc();

            let update_theme = UpdateThemeSetting {
                theme_name: &theme_str,
                updated_at: now,
            };

            diesel::update(theme_settings::table.filter(theme_settings::id.eq(1)))
                .set(&update_theme)
                .execute(&mut *connection)
                .map_err(|e| anyhow::anyhow!("Database update error: {}", e))?;

            Ok(())
        })
        .await??;

        Ok(())
    }
}
