use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::usecases::settings::{
    CreateGithubUseCase, DeleteGithubUseCase, LoadGithubsUseCase, LoadSettingsUseCase,
    SaveThemeUseCase,
};
use infrastructure::{database::DatabaseConnection, repositories::{GithubApiRestRepository, SettingsDbRepository}};
use presentation::commands::{
    create_github, delete_github, load_githubs, load_settings, save_theme,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { run_async().await });
}

async fn run_async() {
    // Database setup
    let database_url = "sqlite://reportify.db";
    let db_connection = DatabaseConnection::new(database_url)
        .await
        .expect("Failed to establish database connection");

    // Dependency injection setup
    let settings_repository = Arc::new(SettingsDbRepository::new(
        db_connection.get_connection().clone(),
    ));
    let github_api_repository = Arc::new(GithubApiRestRepository::new());
    let load_settings_usecase = Arc::new(LoadSettingsUseCase::new(settings_repository.clone()));
    let save_theme_usecase = Arc::new(SaveThemeUseCase::new(settings_repository.clone()));

    // GitHub settings use cases
    let load_githubs_usecase = Arc::new(LoadGithubsUseCase::new(settings_repository.clone()));
    let create_github_usecase = Arc::new(CreateGithubUseCase::new(settings_repository.clone(), github_api_repository.clone()));
    let delete_github_usecase = Arc::new(DeleteGithubUseCase::new(settings_repository.clone()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(load_settings_usecase)
        .manage(save_theme_usecase)
        .manage(load_githubs_usecase)
        .manage(create_github_usecase)
        .manage(delete_github_usecase)
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_theme,
            load_githubs,
            create_github,
            delete_github,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
