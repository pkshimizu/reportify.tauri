use std::sync::Arc;

use anyhow::Result;
use chrono::{DateTime, Utc};

use crate::domain::models::github::GitHubEvent;
use crate::domain::repositories::{
    ActivityRepository, GitHubRepository, GithubApiRepository, SettingsRepository,
};

pub struct FetchGitHubEventsUseCase {
    settings_repository: Arc<dyn SettingsRepository>,
    github_api_repository: Arc<dyn GithubApiRepository>,
    github_repository: Arc<dyn GitHubRepository>,
    activity_repository: Arc<dyn ActivityRepository>,
}

impl FetchGitHubEventsUseCase {
    pub fn new(
        settings_repository: Arc<dyn SettingsRepository>,
        github_api_repository: Arc<dyn GithubApiRepository>,
        github_repository: Arc<dyn GitHubRepository>,
        activity_repository: Arc<dyn ActivityRepository>,
    ) -> Self {
        Self {
            settings_repository,
            github_api_repository,
            github_repository,
            activity_repository,
        }
    }

    pub async fn execute(&self) -> Result<()> {
        let github_accounts = self.settings_repository.load_githubs().await?;

        for account in github_accounts {
            let events: Vec<GitHubEvent> = self
                .github_api_repository
                .get_events(
                    account.username.clone(),
                    account.personal_access_token.clone(),
                    account.latest_event_id.clone(),
                )
                .await?;

            let mut latest_event: Option<&GitHubEvent> = None;

            for event in &events {
                let is_saved = self.github_repository.save_event(event.clone()).await?;

                if is_saved {
                    let activity = event.to_activity();
                    let _ = self.activity_repository.save(activity).await;
                }

                // Track the latest event by created_at timestamp
                if latest_event.is_none() || event.created_at > latest_event.unwrap().created_at {
                    latest_event = Some(event);
                }
            }

            // Save the latest event ID if any events were processed
            if let Some(event) = latest_event {
                let _ = self
                    .settings_repository
                    .save_github_latest_event_id(account.id, event.id.clone())
                    .await;
            }
        }
        Ok(())
    }

    pub async fn execute_with_range(
        &self,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<()> {
        let github_accounts = self.settings_repository.load_githubs().await?;

        for account in github_accounts {
            let repositories = self
                .github_api_repository
                .get_repositories(account.personal_access_token.clone())
                .await?;
            for repository in repositories {
                let events_collection = self
                    .github_api_repository
                    .get_events_collection(
                        repository.owner.username.clone(),
                        repository.name.clone(),
                        account.personal_access_token.clone(),
                        start_date,
                        end_date,
                    )
                    .await?;

                for commit in events_collection.commits {
                    let activity = commit.to_activity();
                    let _ = self.activity_repository.save(activity).await;
                }

                for issue in events_collection.issues {
                    let activity = issue.to_activity();
                    let _ = self.activity_repository.save(activity).await;
                }

                for pull_request in events_collection.pull_requests {
                    let activity = pull_request.to_activity();
                    let _ = self.activity_repository.save(activity).await;
                }

                for review in events_collection.pull_request_reviews {
                    let activity = review.to_activity();
                    let _ = self.activity_repository.save(activity).await;
                }
            }
        }

        Ok(())
    }
}
