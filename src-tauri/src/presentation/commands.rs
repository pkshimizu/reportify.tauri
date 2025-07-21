use std::sync::Arc;

use tauri::State;

use crate::{application::usecases::SettingsUseCase, domain::entities::Theme};

#[tauri::command]
pub async fn get_settings(
    settings_usecase: State<'_, Arc<SettingsUseCase>>,
) -> Result<serde_json::Value, String> {
    match settings_usecase.get_settings().await {
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
pub async fn update_theme(
    theme: String,
    settings_usecase: State<'_, Arc<SettingsUseCase>>,
) -> Result<(), String> {
    let theme = Theme::from_string(&theme).map_err(|e| format!("Invalid theme: {e}"))?;

    settings_usecase
        .update_theme(theme)
        .await
        .map_err(|e| format!("Failed to update theme: {e}"))
}
