use std::sync::Arc;

mod domain;
mod infrastructure;
mod application;
mod presentation;

use application::usecases::ThemeUseCase;
use infrastructure::repositories::InMemoryThemeRepository;
use presentation::commands::{get_theme, update_theme};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Dependency injection setup
    let theme_repository = Arc::new(InMemoryThemeRepository::new());
    let theme_usecase = Arc::new(ThemeUseCase::new(theme_repository));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(theme_usecase)
        .invoke_handler(tauri::generate_handler![
            get_theme,
            update_theme,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
