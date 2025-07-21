use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::usecases::SettingsUseCase;
use infrastructure::{database::DatabaseConnection, repositories::SqliteSettingsRepository};
use presentation::commands::{get_settings, update_theme};

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
    let settings_usecase = Arc::new(SettingsUseCase::new(settings_repository));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(settings_usecase)
        .invoke_handler(tauri::generate_handler![get_settings, update_theme,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
