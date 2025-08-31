use std::sync::Arc;

use anyhow::Result;

use crate::domain::models::settings::SettingsGithubRepository;
use crate::domain::repositories::{GithubApiRepository, SettingsRepository};

pub struct AddGitHubRepositoryUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
    github_api_repository: Arc<dyn GithubApiRepository>,
}

impl AddGitHubRepositoryUseCase {
    pub fn new(
        settings_repository: Arc<dyn SettingsRepository>,
        github_api_repository: Arc<dyn GithubApiRepository>,
    ) -> Self {
        Self {
            settings_repository,
            github_api_repository,
        }
    }

    pub async fn execute(&self, github_repository_id: i32) -> Result<SettingsGithubRepository> {
        // Get all GitHub tokens to find the repository
        let github_settings = self.settings_repository.load_githubs().await?;

        for github_setting in github_settings {
            // Try to get repositories using this token
            match self
                .github_api_repository
                .get_repositories(github_setting.personal_access_token)
                .await
            {
                Ok(repositories) => {
                    // Find the repository with the specified ID
                    if let Some(repository) = repositories
                        .into_iter()
                        .find(|r| r.id == github_repository_id)
                    {
                        // Add the repository to settings
                        return self
                            .settings_repository
                            .add_github_repository(repository)
                            .await;
                    }
                }
                Err(_) => continue, // Try next token if this one fails
            }
        }

        Err(anyhow::anyhow!(
            "Repository with ID {} not found in any accessible GitHub accounts",
            github_repository_id
        ))
    }
}
