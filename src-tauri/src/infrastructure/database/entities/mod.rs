pub mod activity;
pub mod github_event;
pub mod settings;
pub mod settings_github;
pub mod settings_github_repositories;

#[allow(unused_imports)]
pub use activity::Entity as ActivityEntity;
#[allow(unused_imports)]
pub use github_event::Entity as GitHubEventEntity;
pub use settings::Entity as SettingsEntity;
pub use settings_github::Entity as SettingsGithubEntity;
pub use settings_github_repositories::Entity as SettingsGithubRepositoriesEntity;
