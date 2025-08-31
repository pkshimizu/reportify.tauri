use super::Theme;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Settings {
    pub theme: Theme,
    pub language: String,
}

impl Settings {
    pub fn new(theme: Theme, language: String) -> Self {
        Self { theme, language }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsGithub {
    pub id: i32,
    pub username: String,
    #[allow(unused)]
    pub github_id: i32,
    #[allow(unused)]
    pub avatar_url: Option<String>,
    pub personal_access_token: String,
    pub latest_event_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SettingsGithub {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        username: String,
        github_id: i32,
        avatar_url: Option<String>,
        personal_access_token: String,
        latest_event_id: Option<String>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            github_id,
            avatar_url,
            personal_access_token,
            latest_event_id,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SettingsGithubRepository {
    pub id: i32,
    pub github_repository_id: i32,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub owner_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SettingsGithubRepository {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: i32,
        github_repository_id: i32,
        name: String,
        full_name: String,
        description: Option<String>,
        private: bool,
        owner_name: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            github_repository_id,
            name,
            full_name,
            description,
            private,
            owner_name,
            created_at,
            updated_at,
        }
    }
}
