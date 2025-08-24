use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize)]
struct GraphQLRequest {
    query: String,
    variables: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLResponse<T> {
    pub data: Option<T>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Deserialize, Debug)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Deserialize, Debug)]
pub struct RepositoryData {
    pub repository: Option<Repository>,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    #[serde(rename = "defaultBranchRef")]
    pub default_branch_ref: Option<DefaultBranchRef>,
    pub issues: IssueConnection,
    #[serde(rename = "pullRequests")]
    pub pull_requests: PullRequestConnection,
}

#[derive(Deserialize, Debug)]
pub struct DefaultBranchRef {
    pub target: Option<CommitTarget>,
}

#[derive(Deserialize, Debug)]
pub struct CommitTarget {
    pub history: CommitConnection,
}

#[derive(Deserialize, Debug)]
pub struct CommitConnection {
    pub nodes: Vec<Option<CommitNode>>,
    #[serde(rename = "pageInfo")]
    #[allow(unused)]
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct CommitNode {
    pub oid: String,
    pub message: String,
    #[serde(rename = "committedDate")]
    pub committed_date: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct IssueConnection {
    pub nodes: Vec<Option<IssueNode>>,
    #[serde(rename = "pageInfo")]
    #[allow(unused)]
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct IssueNode {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestConnection {
    pub nodes: Vec<Option<PullRequestNode>>,
    #[serde(rename = "pageInfo")]
    #[allow(unused)]
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestNode {
    pub id: String,
    pub title: String,
    pub body: Option<String>,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    pub reviews: Option<PullRequestReviewConnection>,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestReviewConnection {
    pub nodes: Vec<Option<PullRequestReviewNode>>,
    #[serde(rename = "pageInfo")]
    #[allow(unused)]
    pub page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
pub struct PullRequestReviewNode {
    pub id: String,
    pub body: Option<String>,
    pub url: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

#[derive(Deserialize, Debug)]
pub struct PageInfo {
    #[serde(rename = "hasNextPage")]
    #[allow(unused)]
    pub has_next_page: bool,
    #[serde(rename = "endCursor")]
    #[allow(unused)]
    pub end_cursor: Option<String>,
}

pub struct GitHubGraphQLClient {
    client: Client,
}

impl GitHubGraphQLClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn query_repository_events(
        &self,
        owner: String,
        name: String,
        start_date: String,
        end_date: String,
        token: &str,
    ) -> Result<GraphQLResponse<RepositoryData>, anyhow::Error> {
        let query = r#"
            query GetRepositoryEvents($owner: String!, $name: String!, $startDate: DateTime!, $gitStartDate: GitTimestamp!, $gitEndDate: GitTimestamp!, $cursor: String) {
                repository(owner: $owner, name: $name) {
                    defaultBranchRef {
                        target {
                            ... on Commit {
                                history(first: 100, since: $gitStartDate, until: $gitEndDate, after: $cursor) {
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

        let mut variables = HashMap::new();
        variables.insert("owner".to_string(), serde_json::Value::String(owner));
        variables.insert("name".to_string(), serde_json::Value::String(name));
        variables.insert(
            "startDate".to_string(),
            serde_json::Value::String(start_date.clone()),
        );
        variables.insert(
            "gitStartDate".to_string(),
            serde_json::Value::String(start_date),
        );
        variables.insert(
            "gitEndDate".to_string(),
            serde_json::Value::String(end_date),
        );
        variables.insert("cursor".to_string(), serde_json::Value::Null);

        let request_body = GraphQLRequest {
            query: query.to_string(),
            variables,
        };

        let response = self
            .client
            .post("https://api.github.com/graphql")
            .header("Authorization", format!("Bearer {}", token))
            .header("Content-Type", "application/json")
            .header("User-Agent", "reportify/1.0")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GraphQL request failed with status: {}",
                response.status()
            ));
        }

        let response_body: GraphQLResponse<RepositoryData> = response.json().await?;
        Ok(response_body)
    }
}