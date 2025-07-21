use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    domain::{
        models::{Settings, Theme},
        repositories::SettingsRepository,
    },
    infrastructure::database::entities::{settings, SettingsEntity},
};

pub struct DbSettingsRepository {
    db_connection: DatabaseConnection,
}

impl DbSettingsRepository {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl SettingsRepository for DbSettingsRepository {
    async fn load_settings(&self) -> Result<Settings> {
        let settings_record = SettingsEntity::find_by_id(1)
            .one(&self.db_connection)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Settings not found"))?;

        let theme = Theme::from_string(&settings_record.theme)
            .map_err(|e| anyhow::anyhow!("Theme parsing error: {}", e))?;

        Ok(Settings::new(theme, settings_record.language))
    }

    async fn save_theme(&self, theme: Theme) -> Result<()> {
        let settings_record = SettingsEntity::find_by_id(1)
            .one(&self.db_connection)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Settings not found"))?;

        let mut active_model: settings::ActiveModel = settings_record.into();
        active_model.theme = Set(theme.to_string());
        active_model.updated_at = Set(Utc::now());

        active_model.update(&self.db_connection).await?;

        Ok(())
    }
}
