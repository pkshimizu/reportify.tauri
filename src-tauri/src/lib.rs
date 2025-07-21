use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::usecases::ThemeUseCase;
use infrastructure::{database::DatabaseConnection, repositories::SqliteThemeRepository};
use presentation::commands::{get_theme, update_theme};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Database setup
    let database_url = "reportify.db";
    let db_connection =
        DatabaseConnection::new(database_url).expect("Failed to establish database connection");

    // Dependency injection setup
    let theme_repository = Arc::new(SqliteThemeRepository::new(db_connection.get_connection()));
    let theme_usecase = Arc::new(ThemeUseCase::new(theme_repository));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(theme_usecase)
        .invoke_handler(tauri::generate_handler![get_theme, update_theme,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
