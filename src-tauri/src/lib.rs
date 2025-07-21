use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::usecases::{settings::LoadSettingsUseCase, settings::SaveThemeUseCase};
use infrastructure::{database::DatabaseConnection, repositories::SqliteSettingsRepository};
use presentation::commands::{load_settings, save_theme};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Database setup
    let database_url = "reportify.db";
    let db_connection =
        DatabaseConnection::new(database_url).expect("Failed to establish database connection");

    // Dependency injection setup
    let settings_repository = Arc::new(SqliteSettingsRepository::new(
        db_connection.get_connection(),
    ));
    let load_settings_usecase = Arc::new(LoadSettingsUseCase::new(settings_repository.clone()));
    let save_theme_usecase = Arc::new(SaveThemeUseCase::new(settings_repository.clone()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(load_settings_usecase)
        .manage(save_theme_usecase)
        .invoke_handler(tauri::generate_handler![load_settings, save_theme,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
