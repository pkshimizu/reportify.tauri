use std::sync::Arc;

use tauri::State;

use crate::{application::usecases::ThemeUseCase, domain::entities::Theme};

#[tauri::command]
pub async fn get_theme(theme_usecase: State<'_, Arc<ThemeUseCase>>) -> Result<String, String> {
    match theme_usecase.get_theme().await {
        Ok(theme) => Ok(theme.to_string()),
        Err(e) => Err(format!("Failed to get theme: {e}")),
    }
}

#[tauri::command]
pub async fn update_theme(
    theme: String,
    theme_usecase: State<'_, Arc<ThemeUseCase>>,
) -> Result<(), String> {
    let theme = Theme::from_string(&theme).map_err(|e| format!("Invalid theme: {e}"))?;

    theme_usecase
        .update_theme(theme)
        .await
        .map_err(|e| format!("Failed to update theme: {e}"))
}
