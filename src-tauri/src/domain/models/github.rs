#[derive(Debug, Clone)]
pub struct GitHubUser {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GitHubEvent {
    pub id: String,
    pub event_type: String,
    pub actor: GitHubEventActor,
    pub repo: GitHubEventRepo,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GitHubEventActor {
    pub id: i32,
    pub login: String,
    pub avatar_url: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GitHubEventRepo {
    pub id: i32,
    pub name: String,
}
