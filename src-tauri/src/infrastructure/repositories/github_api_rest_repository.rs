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

#[derive(serde::Serialize)]
struct GraphQLQuery {
    query: String,
    variables: serde_json::Value,
}

#[derive(Deserialize)]
struct GraphQLResponse {
    data: Option<RepositoryData>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Deserialize)]
struct RepositoryData {
    repository: Option<Repository>,
}

#[derive(Deserialize)]
struct Repository {
    #[serde(rename = "defaultBranchRef")]
    default_branch_ref: Option<DefaultBranchRef>,
    issues: Connection<IssueNode>,
    #[serde(rename = "pullRequests")]
    pull_requests: Connection<PullRequestNode>,
}

#[derive(Deserialize)]
struct DefaultBranchRef {
    target: Target,
}

#[derive(Deserialize)]
struct Target {
    history: Connection<CommitNode>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Connection<T> {
    nodes: Vec<T>,
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct PageInfo {
    #[serde(rename = "hasNextPage")]
    has_next_page: bool,
    #[serde(rename = "endCursor")]
    end_cursor: Option<String>,
}

#[derive(Deserialize)]
struct CommitNode {
    oid: String,
    message: String,
    #[serde(rename = "committedDate")]
    committed_date: String,
    url: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct IssueNode {
    id: String,
    title: String,
    body: Option<String>,
    url: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    reviews: Option<Connection<ReviewNode>>,
}

#[derive(Deserialize)]
struct PullRequestNode {
    id: String,
    title: String,
    body: Option<String>,
    url: String,
    #[serde(rename = "createdAt")]
    created_at: String,
    reviews: Option<Connection<ReviewNode>>,
}

#[derive(Deserialize)]
struct ReviewNode {
    id: String,
    body: Option<String>,
    url: String,
    #[serde(rename = "createdAt")]
    created_at: String,
}

pub struct GithubApiRestRepository {
    client: GitHubRestApiClient,
}

impl GithubApiRestRepository {
    pub fn new() -> Self {
        Self {
            client: GitHubRestApiClient::new(),
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

        let query = r#"
            query GetRepositoryEvents($owner: String!, $name: String!, $startDate: DateTime!, $endDate: DateTime!, $cursor: String) {
                repository(owner: $owner, name: $name) {
                    defaultBranchRef {
                        target {
                            ... on Commit {
                                history(first: 100, since: $startDate, until: $endDate, after: $cursor) {
                                    nodes {
                                        oid
                                        message
                                        committedDate
                                        url
                                    }
                                    pageInfo {
                                        hasNextPage
                                        endCursor
                                    }
                                }
                            }
                        }
                    }
                    issues(first: 100, states: [OPEN, CLOSED], filterBy: {since: $startDate}) {
                        nodes {
                            id
                            title
                            body
                            url
                            createdAt
                        }
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                    }
                    pullRequests(first: 100, states: [OPEN, CLOSED, MERGED], orderBy: {field: CREATED_AT, direction: DESC}) {
                        nodes {
                            id
                            title
                            body
                            url
                            createdAt
                            reviews(first: 50) {
                                nodes {
                                    id
                                    body
                                    url
                                    createdAt
                                }
                                pageInfo {
                                    hasNextPage
                                    endCursor
                                }
                            }
                        }
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                    }
                }
            }
        "#;

        let variables = serde_json::json!({
            "owner": owner,
            "name": repository_name,
            "startDate": start_date.to_rfc3339(),
            "endDate": end_date.to_rfc3339(),
            "cursor": null
        });

        let query_body = GraphQLQuery {
            query: query.to_string(),
            variables,
        };

        let mut headers = HashMap::new();
        headers.insert(
            "Authorization".to_string(),
            format!("Bearer {personal_access_token}"),
        );
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let response = self
            .client
            .post("/graphql", Some(&query_body), Some(headers))
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub GraphQL API request failed with status: {}",
                response.status()
            ));
        }

        let graphql_response: GraphQLResponse = response.json().await?;

        if let Some(errors) = graphql_response.errors {
            let error_messages: Vec<String> = errors.iter().map(|e| e.message.clone()).collect();
            return Err(anyhow::anyhow!(
                "GraphQL errors: {}",
                error_messages.join(", ")
            ));
        }

        let data = graphql_response
            .data
            .ok_or_else(|| anyhow::anyhow!("No data in GraphQL response"))?;

        let repository = data
            .repository
            .ok_or_else(|| anyhow::anyhow!("Repository not found"))?;

        let commits = self.extract_commits(repository.default_branch_ref, start_date, end_date)?;
        let issues = self.extract_issues(repository.issues, start_date, end_date)?;
        let (pull_requests, pull_request_reviews) =
            self.extract_pull_requests_and_reviews(repository.pull_requests, start_date, end_date)?;

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

    fn extract_commits(
        &self,
        default_branch_ref: Option<DefaultBranchRef>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<GitHubCommit>> {
        let mut commits = Vec::new();

        if let Some(branch_ref) = default_branch_ref {
            for commit_node in branch_ref.target.history.nodes {
                let committed_date =
                    chrono::DateTime::parse_from_rfc3339(&commit_node.committed_date)
                        .map(|dt| dt.with_timezone(&chrono::Utc))
                        .unwrap_or_else(|_| chrono::Utc::now());

                if committed_date >= start_date && committed_date <= end_date {
                    commits.push(GitHubCommit {
                        id: commit_node.oid,
                        message: commit_node.message,
                        url: commit_node.url,
                        created_at: committed_date,
                    });
                }
            }
        }

        Ok(commits)
    }

    fn extract_issues(
        &self,
        issues_connection: Connection<IssueNode>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<GitHubIssue>> {
        let mut issues = Vec::new();

        for issue_node in issues_connection.nodes {
            let created_at = chrono::DateTime::parse_from_rfc3339(&issue_node.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            if created_at >= start_date && created_at <= end_date {
                issues.push(GitHubIssue {
                    id: issue_node.id,
                    title: issue_node.title,
                    body: issue_node.body.unwrap_or_default(),
                    url: issue_node.url,
                    created_at,
                });
            }
        }

        Ok(issues)
    }

    fn extract_pull_requests_and_reviews(
        &self,
        pr_connection: Connection<PullRequestNode>,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<(Vec<GitHubPullRequest>, Vec<GitHubPullRequestReview>)> {
        let mut pull_requests = Vec::new();
        let mut pull_request_reviews = Vec::new();

        for pr_node in pr_connection.nodes {
            let created_at = chrono::DateTime::parse_from_rfc3339(&pr_node.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now());

            if created_at >= start_date && created_at <= end_date {
                pull_requests.push(GitHubPullRequest {
                    id: pr_node.id.clone(),
                    title: pr_node.title,
                    body: pr_node.body.unwrap_or_default(),
                    url: pr_node.url,
                    created_at,
                });

                if let Some(reviews) = pr_node.reviews {
                    for review_node in reviews.nodes {
                        let review_created_at =
                            chrono::DateTime::parse_from_rfc3339(&review_node.created_at)
                                .map(|dt| dt.with_timezone(&chrono::Utc))
                                .unwrap_or_else(|_| chrono::Utc::now());

                        if review_created_at >= start_date && review_created_at <= end_date {
                            pull_request_reviews.push(GitHubPullRequestReview {
                                id: review_node.id,
                                body: review_node.body.unwrap_or_default(),
                                url: review_node.url,
                                created_at: review_created_at,
                            });
                        }
                    }
                }
            }
        }

        Ok((pull_requests, pull_request_reviews))
    }
}
