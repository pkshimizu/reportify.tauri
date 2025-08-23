use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

use crate::domain::models::github::{
    GitHubCommit, GitHubEvent, GitHubEventActor, GitHubEventRepo, GitHubEventsCollection,
    GitHubIssue, GitHubPullRequest, GitHubPullRequestReview, GitHubUser,
};
use crate::domain::repositories::GithubApiRepository;
use crate::infrastructure::clients::GitHubRestApiClient;
use crate::infrastructure::graphql::{GitHubGraphQLClient, RepositoryData, DefaultBranchRef, CommitTarget, IssueConnection, PullRequestConnection};

#[derive(Deserialize)]
struct GitHubApiUser {
    id: i32,
    login: String,
    avatar_url: String,
}

#[derive(Deserialize)]
struct GitHubApiEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    actor: GitHubApiEventActor,
    repo: GitHubApiEventRepo,
    payload: serde_json::Value,
    public: bool,
    created_at: String,
}

#[derive(Deserialize)]
struct GitHubApiEventActor {
    id: i32,
    login: String,
    avatar_url: String,
}

#[derive(Deserialize)]
struct GitHubApiEventRepo {
    id: i32,
    name: String,
}


pub struct GithubApiRestRepository {
    client: GitHubRestApiClient,
    graphql_client: GitHubGraphQLClient,
}

impl GithubApiRestRepository {
    pub fn new() -> Self {
        Self {
            client: GitHubRestApiClient::new(),
            graphql_client: GitHubGraphQLClient::new(),
        }
    }
}

#[async_trait::async_trait]
impl GithubApiRepository for GithubApiRestRepository {
    async fn get_user(&self, personal_access_token: String) -> Result<GitHubUser> {
        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("token {personal_access_token}"),
        );

        let response = self.client.get("/user", Some(headers)).await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let github_user: GitHubApiUser = response.json().await?;

        Ok(GitHubUser {
            id: github_user.id,
            username: github_user.login,
            avatar_url: Some(github_user.avatar_url),
        })
    }

    async fn get_events(
        &self,
        username: String,
        personal_access_token: String,
        latest_event_id: Option<String>,
    ) -> Result<Vec<GitHubEvent>> {
        let mut page = 1;
        let mut results = Vec::new();
        loop {
            let path = format!("/users/{username}/events?per_page=100&page={page}");

            let mut headers = HashMap::new();
            headers.insert(
                "Authorization".to_string(),
                format!("token {personal_access_token}"),
            );

            let response = self.client.get(&path, Some(headers)).await?;

            if !response.status().is_success() {
                if response.status().as_u16() == 422 {
                    break;
                }
                return Err(anyhow::anyhow!(
                    "GitHub API request failed with status: {}",
                    response.status()
                ));
            }

            let github_events: Vec<GitHubApiEvent> = response.json().await?;

            let events: Vec<GitHubEvent> = github_events
                .into_iter()
                .map(|event| GitHubEvent {
                    id: event.id,
                    event_type: event.event_type,
                    actor: GitHubEventActor {
                        id: event.actor.id,
                        login: event.actor.login,
                        avatar_url: event.actor.avatar_url,
                    },
                    repo: GitHubEventRepo {
                        id: event.repo.id,
                        name: event.repo.name,
                    },
                    payload: event.payload,
                    public: event.public,
                    created_at: chrono::DateTime::parse_from_rfc3339(&event.created_at)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now()),
                })
                .collect();
            if events.is_empty() {
                break;
            }
            results.extend(events.clone());
            if let Some(unwrapped_latest_event_id) = latest_event_id.clone() {
                if events.iter().any(|e| e.id == unwrapped_latest_event_id) {
                    break;
                }
            }
            page += 1;
        }

        Ok(results)
    }

    async fn get_events_collection(
        &self,
        username: String,
        personal_access_token: String,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<GitHubEventsCollection> {
        let repository_name = self.extract_repository_name(username.clone())?;
        let owner = self.extract_owner(username)?;

        let response = self
            .graphql_client
            .query_repository_events(
                owner,
                repository_name,
                start_date.to_rfc3339(),
                end_date.to_rfc3339(),
                &personal_access_token,
            )
            .await?;

        if let Some(errors) = response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(anyhow::anyhow!(
                "GraphQL errors: {}",
                error_messages.join(", ")
            ));
        }

        let data = response
            .data
            .ok_or_else(|| anyhow::anyhow!("No data in GraphQL response"))?;

        let repository = data
            .repository
            .ok_or_else(|| anyhow::anyhow!("Repository not found"))?;

        let commits = self.extract_commits_from_response(repository.default_branch_ref, start_date, end_date)?;
        let issues = self.extract_issues_from_response(repository.issues, start_date, end_date)?;
        let (pull_requests, pull_request_reviews) =
            self.extract_pull_requests_and_reviews_from_response(repository.pull_requests, start_date, end_date)?;

        Ok(GitHubEventsCollection {
            commits,
            issues,
            pull_requests,
            pull_request_reviews,
        })
    }
}

impl GithubApiRestRepository {
    fn extract_repository_name(&self, username: String) -> Result<String> {
        if username.contains('/') {
            Ok(username.split('/').nth(1).unwrap_or("").to_string())
        } else {
            Err(anyhow::anyhow!(
                "Invalid username format. Expected 'owner/repo'"
            ))
        }
    }

    fn extract_owner(&self, username: String) -> Result<String> {
        if username.contains('/') {
            Ok(username.split('/').next().unwrap_or("").to_string())
        } else {
            Err(anyhow::anyhow!(
                "Invalid username format. Expected 'owner/repo'"
            ))
        }
    }

    fn extract_commits_from_response(
        &self,
        default_branch_ref: Option<DefaultBranchRef>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<GitHubCommit>> {
        let mut commits = Vec::new();

        if let Some(branch_ref) = default_branch_ref {
            if let Some(commit_target) = branch_ref.target {
                for commit_node in commit_target.history.nodes {
                    if let Some(node) = commit_node {
                        let committed_date = chrono::DateTime::parse_from_rfc3339(&node.committed_date)
                            .map(|dt| dt.with_timezone(&chrono::Utc))
                            .unwrap_or_else(|_| chrono::Utc::now());

                        if committed_date >= start_date && committed_date <= end_date {
                            commits.push(GitHubCommit {
                                id: node.oid,
                                message: node.message,
                                url: node.url,
                                created_at: committed_date,
                            });
                        }
                    }
                }
            }
        }

        Ok(commits)
    }

    fn extract_issues_from_response(
        &self,
        issues_connection: IssueConnection,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<GitHubIssue>> {
        let mut issues = Vec::new();

        for issue_node in issues_connection.nodes {
            if let Some(node) = issue_node {
                let created_at = chrono::DateTime::parse_from_rfc3339(&node.created_at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                if created_at >= start_date && created_at <= end_date {
                    issues.push(GitHubIssue {
                        id: node.id,
                        title: node.title,
                        body: node.body.unwrap_or_default(),
                        url: node.url,
                        created_at,
                    });
                }
            }
        }

        Ok(issues)
    }

    fn extract_pull_requests_and_reviews_from_response(
        &self,
        pr_connection: PullRequestConnection,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<(Vec<GitHubPullRequest>, Vec<GitHubPullRequestReview>)> {
        let mut pull_requests = Vec::new();
        let mut pull_request_reviews = Vec::new();

        for pr_node in pr_connection.nodes {
            if let Some(node) = pr_node {
                let created_at = chrono::DateTime::parse_from_rfc3339(&node.created_at)
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .unwrap_or_else(|_| chrono::Utc::now());

                if created_at >= start_date && created_at <= end_date {
                    pull_requests.push(GitHubPullRequest {
                        id: node.id.clone(),
                        title: node.title,
                        body: node.body.unwrap_or_default(),
                        url: node.url,
                        created_at,
                    });

                    if let Some(reviews) = node.reviews {
                        for review_node in reviews.nodes {
                            if let Some(review) = review_node {
                                let review_created_at =
                                    chrono::DateTime::parse_from_rfc3339(&review.created_at)
                                        .map(|dt| dt.with_timezone(&chrono::Utc))
                                        .unwrap_or_else(|_| chrono::Utc::now());

                                if review_created_at >= start_date && review_created_at <= end_date {
                                    pull_request_reviews.push(GitHubPullRequestReview {
                                        id: review.id,
                                        body: review.body.unwrap_or_default(),
                                        url: review.url,
                                        created_at: review_created_at,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok((pull_requests, pull_request_reviews))
    }
}
