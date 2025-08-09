use chrono::{DateTime, Utc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Activity {
    pub service: String,
    pub activity_type: String,
    pub summary: String,
    pub detail: String,
    pub original_url: Option<String>,
    pub created_at: DateTime<Utc>,
}
