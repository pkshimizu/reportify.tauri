use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

use crate::{
    domain::{
        models::{
            settings::{Settings, SettingsGithub},
            GitHubUser, Theme,
        },
        repositories::SettingsRepository,
    },
    infrastructure::database::entities::{
        settings, settings_github, SettingsEntity, SettingsGithubEntity,
    },
};

pub struct SettingsDbRepository {
    db_connection: DatabaseConnection,
}

impl SettingsDbRepository {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl SettingsRepository for SettingsDbRepository {
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

    async fn load_githubs(&self) -> Result<Vec<SettingsGithub>> {
        let github_settings = SettingsGithubEntity::find()
            .all(&self.db_connection)
            .await?;

        let settings = github_settings
            .into_iter()
            .map(|record| {
                SettingsGithub::new(
                    record.id,
                    record.username,
                    record.github_id,
                    record.avatar_url,
                    record.personal_access_token,
                    record.latest_event_id,
                    record.created_at,
                    record.updated_at,
                )
            })
            .collect();

        Ok(settings)
    }

    async fn create_github(
        &self,
        user: GitHubUser,
        personal_access_token: String,
    ) -> Result<SettingsGithub> {
        let now = Utc::now();
        let active_model = settings_github::ActiveModel {
            id: sea_orm::NotSet,
            username: Set(user.username),
            github_id: Set(user.id),
            avatar_url: Set(user.avatar_url),
            personal_access_token: Set(personal_access_token),
            latest_event_id: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        };

        let result = active_model.insert(&self.db_connection).await?;

        Ok(SettingsGithub::new(
            result.id,
            result.username,
            result.github_id,
            result.avatar_url,
            result.personal_access_token,
            result.latest_event_id,
            result.created_at,
            result.updated_at,
        ))
    }

    async fn delete_github(&self, id: i32) -> Result<()> {
        let github_setting = SettingsGithubEntity::find_by_id(id)
            .one(&self.db_connection)
            .await?
            .ok_or_else(|| anyhow::anyhow!("GitHub setting not found"))?;

        SettingsGithubEntity::delete_by_id(github_setting.id)
            .exec(&self.db_connection)
            .await?;

        Ok(())
    }
}
