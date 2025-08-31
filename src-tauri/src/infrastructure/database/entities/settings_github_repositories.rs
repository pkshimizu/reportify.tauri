use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "settings_github_repositories")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub github_repository_id: i32,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub owner_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
