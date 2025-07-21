use std::sync::Arc;

use tauri::State;

use crate::application::usecases::{settings::LoadSettingsUseCase, settings::SaveThemeUseCase};
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
