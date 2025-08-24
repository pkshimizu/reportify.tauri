use super::activity::Activity;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct GitHubUser {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GitHubRepository {
    #[allow(unused)]
    pub id: i32,
    pub owner: GitHubUser,
    pub name: String,
    #[allow(unused)]
    pub full_name: String,
    #[allow(unused)]
    pub description: Option<String>,
    #[allow(unused)]
    pub html_url: String,
    #[allow(unused)]
    pub private: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct GitHubEvent {
    pub id: String,
    pub event_type: String,
    pub actor: GitHubEventActor,
    pub repo: GitHubEventRepo,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct GitHubEventActor {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[allow(dead_code)]
pub struct GitHubEventRepo {
    pub id: i32,
    pub name: String,
}

impl GitHubEvent {
    pub fn to_activity(&self) -> Activity {
        let (summary, detail) = self.extract_summary_and_detail();
        let original_url = self.generate_original_url();

        Activity {
            service: "github".to_string(),
            activity_type: self.event_type.clone(),
            summary,
            detail,
            original_url,
            created_at: self.created_at,
        }
    }

    fn extract_summary_and_detail(&self) -> (String, String) {
        match self.event_type.as_str() {
            "PushEvent" => {
                let commits_count = self.payload["commits"]
                    .as_array()
                    .map(|c| c.len())
                    .unwrap_or(0);
                let summary = if commits_count == 1 {
                    "Pushed commit".to_string()
                } else {
                    format!("Pushed {commits_count} commits")
                };
                let detail = self.payload["commits"]
                    .as_array()
                    .and_then(|commits| commits.first())
                    .and_then(|commit| commit["message"].as_str())
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "CreateEvent" => {
                let ref_type = self.payload["ref_type"].as_str().unwrap_or("repository");
                let summary = match ref_type {
                    "repository" => "Created repository".to_string(),
                    "branch" => "Created branch".to_string(),
                    "tag" => "Created tag".to_string(),
                    _ => format!("Created {ref_type}"),
                };
                (summary, String::new())
            }
            "DeleteEvent" => {
                let ref_type = self.payload["ref_type"].as_str().unwrap_or("branch");
                let summary = match ref_type {
                    "branch" => "Deleted branch".to_string(),
                    "tag" => "Deleted tag".to_string(),
                    _ => format!("Deleted {ref_type}"),
                };
                (summary, String::new())
            }
            "IssuesEvent" => {
                let action = self.payload["action"].as_str().unwrap_or("updated");
                let summary = match action {
                    "opened" => "Opened issue".to_string(),
                    "closed" => "Closed issue".to_string(),
                    "reopened" => "Reopened issue".to_string(),
                    _ => format!(
                        "{} issue",
                        action
                            .chars()
                            .next()
                            .unwrap()
                            .to_uppercase()
                            .collect::<String>()
                            + &action[1..]
                    ),
                };
                let detail = self.payload["issue"]["title"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "IssueCommentEvent" => {
                let summary = "Created issue comment".to_string();
                let detail = self.payload["comment"]["body"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "PullRequestEvent" => {
                let action = self.payload["action"].as_str().unwrap_or("updated");
                let summary = match action {
                    "opened" => "Opened pull request".to_string(),
                    "closed" => "Closed pull request".to_string(),
                    "reopened" => "Reopened pull request".to_string(),
                    _ => format!(
                        "{} pull request",
                        action
                            .chars()
                            .next()
                            .unwrap()
                            .to_uppercase()
                            .collect::<String>()
                            + &action[1..]
                    ),
                };
                let detail = self.payload["pull_request"]["title"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "PullRequestReviewEvent" => {
                let summary = "Reviewed pull request".to_string();
                let detail = self.payload["review"]["body"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "PullRequestReviewCommentEvent" => {
                let summary = "Created pull request review comment".to_string();
                let detail = self.payload["comment"]["body"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "CommitCommentEvent" => {
                let summary = "Created commit comment".to_string();
                let detail = self.payload["comment"]["body"]
                    .as_str()
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            "ForkEvent" => ("Forked repository".to_string(), String::new()),
            "WatchEvent" => ("Starred repository".to_string(), String::new()),
            "ReleaseEvent" => {
                let action = self.payload["action"].as_str().unwrap_or("published");
                let summary = match action {
                    "published" => "Published release".to_string(),
                    _ => format!(
                        "{} release",
                        action
                            .chars()
                            .next()
                            .unwrap()
                            .to_uppercase()
                            .collect::<String>()
                            + &action[1..]
                    ),
                };
                let detail = self.payload["release"]["name"]
                    .as_str()
                    .or_else(|| self.payload["release"]["tag_name"].as_str())
                    .unwrap_or("")
                    .to_string();
                (summary, detail)
            }
            _ => (format!("GitHub {}", self.event_type), String::new()),
        }
    }

    fn generate_original_url(&self) -> Option<String> {
        let repo_name = &self.repo.name;
        match self.event_type.as_str() {
            "PushEvent" => {
                // Link to the first commit in the push
                self.payload["commits"]
                    .as_array()
                    .and_then(|commits| commits.first())
                    .and_then(|commit| commit["sha"].as_str())
                    .map(|sha| format!("https://github.com/{repo_name}/commit/{sha}"))
            }
            "IssuesEvent" | "IssueCommentEvent" => self.payload["issue"]["html_url"]
                .as_str()
                .map(|url| url.to_string()),
            "PullRequestEvent" | "PullRequestReviewEvent" | "PullRequestReviewCommentEvent" => self
                .payload["pull_request"]["html_url"]
                .as_str()
                .map(|url| url.to_string()),
            "CommitCommentEvent" => self.payload["comment"]["html_url"]
                .as_str()
                .map(|url| url.to_string()),
            "ReleaseEvent" => self.payload["release"]["html_url"]
                .as_str()
                .map(|url| url.to_string()),
            "CreateEvent" | "DeleteEvent" => {
                let ref_type = self.payload["ref_type"].as_str().unwrap_or("");
                let ref_name = self.payload["ref"].as_str().unwrap_or("");

                match ref_type {
                    "branch" if !ref_name.is_empty() => {
                        Some(format!("https://github.com/{repo_name}/tree/{ref_name}"))
                    }
                    "tag" if !ref_name.is_empty() => Some(format!(
                        "https://github.com/{repo_name}/releases/tag/{ref_name}"
                    )),
                    _ => Some(format!("https://github.com/{repo_name}")),
                }
            }
            "ForkEvent" => self.payload["forkee"]["html_url"]
                .as_str()
                .map(|url| url.to_string()),
            _ => Some(format!("https://github.com/{repo_name}")),
        }
    }
}

#[allow(dead_code)]
pub struct GitHubCommit {
    pub id: String,
    pub message: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl GitHubCommit {
    pub fn to_activity(&self) -> Activity {
        Activity {
            service: "github".to_string(),
            activity_type: "PushEvent".to_string(),
            summary: "Pushed commit".to_string(),
            detail: self.message.clone(),
            original_url: Some(self.url.clone()),
            created_at: self.created_at,
        }
    }
}

#[allow(dead_code)]
pub struct GitHubIssue {
    pub id: String,
    pub title: String,
    pub body: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl GitHubIssue {
    pub fn to_activity(&self) -> Activity {
        Activity {
            service: "github".to_string(),
            activity_type: "IssuesEvent".to_string(),
            summary: "Opened issue".to_string(),
            detail: self.title.clone(),
            original_url: Some(self.url.clone()),
            created_at: self.created_at,
        }
    }
}

#[allow(dead_code)]
pub struct GitHubPullRequest {
    pub id: String,
    pub title: String,
    pub body: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl GitHubPullRequest {
    pub fn to_activity(&self) -> Activity {
        Activity {
            service: "github".to_string(),
            activity_type: "PullRequestEvent".to_string(),
            summary: "Opened pull request".to_string(),
            detail: self.title.clone(),
            original_url: Some(self.url.clone()),
            created_at: self.created_at,
        }
    }
}

#[allow(dead_code)]
pub struct GitHubPullRequestReview {
    pub id: String,
    pub body: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl GitHubPullRequestReview {
    pub fn to_activity(&self) -> Activity {
        Activity {
            service: "github".to_string(),
            activity_type: "PullRequestReviewEvent".to_string(),
            summary: "Reviewed pull request".to_string(),
            detail: self.body.clone(),
            original_url: Some(self.url.clone()),
            created_at: self.created_at,
        }
    }
}

#[allow(dead_code)]
pub struct GitHubEventsCollection {
    pub commits: Vec<GitHubCommit>,
    pub issues: Vec<GitHubIssue>,
    pub pull_requests: Vec<GitHubPullRequest>,
    pub pull_request_reviews: Vec<GitHubPullRequestReview>,
}
