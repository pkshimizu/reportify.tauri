use std::sync::Arc;

use tauri::State;

use crate::application::usecases::activities::LoadActivitiesUseCase;
use crate::application::usecases::fetcher::FetchGitHubEventsUseCase;
use crate::application::usecases::settings::{
    CreateGithubUseCase, DeleteGithubUseCase, LoadGithubsUseCase, LoadSettingsUseCase,
    SaveThemeUseCase,
};
use crate::domain::models::Theme;

#[tauri::command]
pub async fn load_settings(
    load_settings_usecase: State<'_, Arc<LoadSettingsUseCase>>,
) -> Result<serde_json::Value, String> {
    match load_settings_usecase.execute().await {
        Ok(settings) => {
            let json = serde_json::json!({
                "theme": settings.theme.to_string(),
                "language": settings.language
            });
            Ok(json)
        }
        Err(e) => Err(format!("Failed to get settings: {e}")),
    }
}

#[tauri::command]
pub async fn save_theme(
    theme: String,
    save_theme_usecase: State<'_, Arc<SaveThemeUseCase>>,
) -> Result<(), String> {
    let theme = Theme::from_string(&theme).map_err(|e| format!("Invalid theme: {e}"))?;

    save_theme_usecase
        .execute(theme)
        .await
        .map_err(|e| format!("Failed to update theme: {e}"))
}

#[tauri::command]
pub async fn load_githubs(
    load_githubs_usecase: State<'_, Arc<LoadGithubsUseCase>>,
) -> Result<serde_json::Value, String> {
    match load_githubs_usecase.execute().await {
        Ok(githubs) => {
            let json_githubs: Vec<serde_json::Value> = githubs
                .into_iter()
                .map(|github| {
                    serde_json::json!({
                        "id": github.id,
                        "personalAccessToken": github.personal_access_token,
                        "createdAt": github.created_at.to_rfc3339(),
                        "updatedAt": github.updated_at.to_rfc3339()
                    })
                })
                .collect();
            Ok(serde_json::json!(json_githubs))
        }
        Err(e) => Err(format!("Failed to load GitHub settings: {e}")),
    }
}

#[tauri::command]
pub async fn create_github(
    personal_access_token: String,
    create_github_usecase: State<'_, Arc<CreateGithubUseCase>>,
) -> Result<serde_json::Value, String> {
    match create_github_usecase.execute(personal_access_token).await {
        Ok(github) => {
            let json = serde_json::json!({
                "id": github.id,
                "personalAccessToken": github.personal_access_token,
                "createdAt": github.created_at.to_rfc3339(),
                "updatedAt": github.updated_at.to_rfc3339()
            });
            Ok(json)
        }
        Err(e) => Err(format!("Failed to create GitHub setting: {e}")),
    }
}

#[tauri::command]
pub async fn delete_github(
    id: i32,
    delete_github_usecase: State<'_, Arc<DeleteGithubUseCase>>,
) -> Result<(), String> {
    delete_github_usecase
        .execute(id)
        .await
        .map_err(|e| format!("Failed to delete GitHub setting: {e}"))
}

#[tauri::command]
pub async fn fetch_github_events(
    fetch_github_events_usecase: State<'_, Arc<FetchGitHubEventsUseCase>>,
) -> Result<(), String> {
    fetch_github_events_usecase
        .execute()
        .await
        .map_err(|e| format!("Failed to fetch GitHub events: {e}"))
}

#[tauri::command]
pub async fn load_activities(
    year: i32,
    month: u32,
    load_activities_usecase: State<'_, Arc<LoadActivitiesUseCase>>,
) -> Result<serde_json::Value, String> {
    match load_activities_usecase.execute(year, month).await {
        Ok(activities) => {
            let json_activities: Vec<serde_json::Value> = activities
                .into_iter()
                .map(|activity| {
                    serde_json::json!({
                        "service": activity.service,
                        "activityType": activity.activity_type,
                        "summary": activity.summary,
                        "detail": activity.detail,
                        "originalUrl": activity.original_url,
                        "createdAt": activity.created_at.to_rfc3339()
                    })
                })
                .collect();
            Ok(serde_json::json!(json_activities))
        }
        Err(e) => Err(format!("Failed to load activities: {e}")),
    }
}
