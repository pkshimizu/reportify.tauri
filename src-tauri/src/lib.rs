use std::sync::Arc;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use application::usecases::activities::LoadActivitiesUseCase;
use application::usecases::fetcher::FetchGitHubEventsUseCase;
use application::usecases::github::LoadGithubRepositoriesUseCase;
use application::usecases::settings::{
    AddGitHubRepositoryUseCase, CreateGithubUseCase, DeleteGithubUseCase, LoadGithubsUseCase,
    LoadSettingsUseCase, RemoveGitHubRepositoryUseCase, SaveThemeUseCase,
};
use infrastructure::{
    database::DatabaseConnection,
    repositories::{
        ActivityDbRepository, GitHubDbRepository, GithubApiRestRepository, SettingsDbRepository,
    },
};
use presentation::commands::{
    add_github_repository, create_github, delete_github, fetch_github_events,
    fetch_github_events_with_range, load_activities, load_github_repositories, load_githubs,
    load_settings, remove_github_repository, save_theme,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async { run_async().await });
}

async fn run_async() {
    // Database setup - use platform-specific app data directory
    let app_data_path = dirs::data_dir()
        .expect("Failed to get data directory")
        .join("reportify");

    std::fs::create_dir_all(&app_data_path).expect("Failed to create app data directory");

    let database_path = app_data_path.join("reportify.db");
    let database_url = format!("sqlite://{}?mode=rwc", database_path.display());
    log::info!("Database URL: {database_url}");

    let db_connection = DatabaseConnection::new(&database_url)
        .await
        .expect("Failed to establish database connection");

    // Dependency injection setup
    let settings_repository = Arc::new(SettingsDbRepository::new(
        db_connection.get_connection().clone(),
    ));
    let github_api_repository = Arc::new(GithubApiRestRepository::new());
    let github_repository = Arc::new(GitHubDbRepository::new(
        db_connection.get_connection().clone(),
    ));
    let activity_repository = Arc::new(ActivityDbRepository::new(
        db_connection.get_connection().clone(),
    ));
    let load_settings_usecase = Arc::new(LoadSettingsUseCase::new(settings_repository.clone()));
    let save_theme_usecase = Arc::new(SaveThemeUseCase::new(settings_repository.clone()));

    // GitHub settings use cases
    let load_githubs_usecase = Arc::new(LoadGithubsUseCase::new(settings_repository.clone()));
    let create_github_usecase = Arc::new(CreateGithubUseCase::new(
        settings_repository.clone(),
        github_api_repository.clone(),
    ));
    let delete_github_usecase = Arc::new(DeleteGithubUseCase::new(settings_repository.clone()));
    let fetch_github_events_usecase = Arc::new(FetchGitHubEventsUseCase::new(
        settings_repository.clone(),
        github_api_repository.clone(),
        github_repository.clone(),
        activity_repository.clone(),
    ));
    let load_activities_usecase = Arc::new(LoadActivitiesUseCase::new(activity_repository.clone()));
    let load_github_repositories_usecase = Arc::new(LoadGithubRepositoriesUseCase::new(
        github_api_repository.clone(),
        settings_repository.clone(),
    ));
    let add_github_repository_usecase = Arc::new(AddGitHubRepositoryUseCase::new(
        settings_repository.clone(),
        github_api_repository.clone(),
    ));
    let remove_github_repository_usecase = Arc::new(RemoveGitHubRepositoryUseCase::new(
        settings_repository.clone(),
    ));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(load_settings_usecase)
        .manage(save_theme_usecase)
        .manage(load_githubs_usecase)
        .manage(create_github_usecase)
        .manage(delete_github_usecase)
        .manage(fetch_github_events_usecase)
        .manage(load_activities_usecase)
        .manage(load_github_repositories_usecase)
        .manage(add_github_repository_usecase)
        .manage(remove_github_repository_usecase)
        .invoke_handler(tauri::generate_handler![
            load_settings,
            save_theme,
            load_githubs,
            create_github,
            delete_github,
            fetch_github_events,
            fetch_github_events_with_range,
            load_activities,
            load_github_repositories,
            add_github_repository,
            remove_github_repository,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
