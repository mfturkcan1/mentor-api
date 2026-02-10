use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, PartialEq, Serialize, ToSchema)]
pub struct TodoSelectDto {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub parent_goal_id: Option<i32>,
    pub completed: bool,
    pub completed_date: Option<DateTime<Utc>>,
    pub deadline_date: DateTime<Utc>,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub delete_date: Option<DateTime<Utc>>,
}
