pub mod activity;
pub mod settings;
pub mod settings_github;

#[allow(unused_imports)]
pub use activity::Entity as ActivityEntity;
pub use settings::Entity as SettingsEntity;
pub use settings_github::Entity as SettingsGithubEntity;
