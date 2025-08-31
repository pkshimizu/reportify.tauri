use std::{collections::HashSet, sync::Arc};

use anyhow::Result;

use crate::domain::models::github::GitHubRepository;
use crate::domain::repositories::{GithubApiRepository, SettingsRepository};

pub struct LoadGithubRepositoriesUseCase {
    github_api_repository: Arc<dyn GithubApiRepository>,
    settings_repository: Arc<dyn SettingsRepository>,
}

impl LoadGithubRepositoriesUseCase {
    pub fn new(
        github_api_repository: Arc<dyn GithubApiRepository>,
        settings_repository: Arc<dyn SettingsRepository>,
    ) -> Self {
        Self {
            github_api_repository,
            settings_repository,
        }
    }

    pub async fn execute(&self) -> Result<Vec<GitHubRepository>> {
        let github_settings = self.settings_repository.load_githubs().await?;

        let mut all_repositories = Vec::new();
        let mut seen_repository_ids = HashSet::new();

        for github_setting in github_settings {
            let repositories = self
                .github_api_repository
                .get_repositories(github_setting.personal_access_token)
                .await?;

            for repository in repositories {
                if seen_repository_ids.insert(repository.id) {
                    all_repositories.push(repository);
                }
            }
        }

        Ok(all_repositories)
    }
}
