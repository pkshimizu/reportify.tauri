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
    pub github_id: i32,
    pub avatar_url: Option<String>,
    pub personal_access_token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SettingsGithub {
    pub fn new(
        id: i32,
        username: String,
        github_id: i32,
        avatar_url: Option<String>,
        personal_access_token: String,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            username,
            github_id,
            avatar_url,
            personal_access_token,
            created_at,
            updated_at,
        }
    }
}
