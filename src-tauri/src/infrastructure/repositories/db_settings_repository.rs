use std::sync::{Arc, Mutex};

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use diesel::prelude::*;

use crate::{
    domain::{
        entities::{Settings, Theme},
        repositories::SettingsRepository,
    },
    infrastructure::database::{models::*, schema::settings},
};

pub struct DbSettingsRepository {
    db_connection: Arc<Mutex<SqliteConnection>>,
}

impl DbSettingsRepository {
    pub fn new(db_connection: Arc<Mutex<SqliteConnection>>) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl SettingsRepository for DbSettingsRepository {
    async fn load_settings(&self) -> Result<Settings> {
        let conn = Arc::clone(&self.db_connection);
        let result = tokio::task::spawn_blocking(move || -> Result<Settings> {
            let mut connection = conn.lock().unwrap();

            let settings_record: crate::infrastructure::database::models::Settings =
                settings::table
                    .filter(settings::id.eq(1))
                    .first(&mut *connection)
                    .map_err(|e| anyhow::anyhow!("Database error: {}", e))?;

            let theme = Theme::from_string(&settings_record.theme)
                .map_err(|e| anyhow::anyhow!("Theme parsing error: {}", e))?;

            Ok(Settings::new(theme, settings_record.language))
        })
        .await??;

        Ok(result)
    }

    async fn save_theme(&self, theme: Theme) -> Result<()> {
        let conn = Arc::clone(&self.db_connection);
        let theme_str = theme.to_string();

        tokio::task::spawn_blocking(move || -> Result<()> {
            let mut connection = conn.lock().unwrap();
            let now = Utc::now().naive_utc();

            let update_settings = UpdateSettings {
                theme: Some(&theme_str),
                language: None,
                updated_at: now,
            };

            diesel::update(settings::table.filter(settings::id.eq(1)))
                .set(&update_settings)
                .execute(&mut *connection)
                .map_err(|e| anyhow::anyhow!("Database update error: {}", e))?;

            Ok(())
        })
        .await??;

        Ok(())
    }
}
