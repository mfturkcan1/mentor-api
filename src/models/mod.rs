use crate::schema::{categories, goals, routine_parts, routines};
use chrono::{DateTime, Utc};
use diesel::deserialize::QueryableByName;
use diesel::sql_types::{BigInt, Text, Timestamptz};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = routines)]
pub struct Routine {
    pub id: i32,
    pub title: String,
    pub create_date: DateTime<Utc>,
    pub update_date: Option<DateTime<Utc>>,
    pub delete_date: Option<DateTime<Utc>>,
}

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[diesel(table_name = routines)]
pub struct NewRoutine<'a> {
    pub title: &'a str,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub delete_date: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq, Serialize, Clone)]
#[diesel(belongs_to(Routine, foreign_key= routine_id))]
#[diesel(table_name = routine_parts)]
pub struct RoutinePart {
    pub id: i32,
    pub description: String,
    pub start_hour: DateTime<Utc>,
    pub end_hour: DateTime<Utc>,
    pub routine_id: i32,
    pub delete_date: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Routine, foreign_key= routine_id))]
#[diesel(table_name = routine_parts)]
pub struct NewRoutinePart<'a> {
    pub description: &'a str,
    pub start_hour: DateTime<Utc>,
    pub end_hour: DateTime<Utc>,
    pub routine_id: i32,
}

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Insertable, Debug, PartialEq, Eq)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
}

impl Hash for NewCategory {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::GoalPeriodType"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum GoalPeriodType {
    Month,
    Year,
    Deadline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::GoalStatus"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum GoalStatus {
    Planned,
    Active,
    Done,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::GoalPriority"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum GoalPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::GoalLifeCycle"]
#[DbValueStyle = "SCREAMING_SNAKE_CASE"]
pub enum GoalLifeCycle {
    ShortTerm,
    MediumTerm,
    LongTerm,
    LifeTime,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = goals)]
pub struct Goals {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub period_type: GoalPeriodType,
    pub deadline_at: Option<DateTime<Utc>>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub status: GoalStatus,
    pub priority: GoalPriority,
    pub target_value: Option<i32>,
    pub current_value: i32,
    pub unit: Option<String>,
    pub parent_goal_id: Option<i32>,
    pub goal_cycle: GoalLifeCycle,
    pub create_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub delete_date: Option<DateTime<Utc>>,
}

#[derive(Insertable, Debug, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = goals)]
pub struct NewGoal {
    pub title: String,
    pub description: Option<String>,
    pub period_type: GoalPeriodType,
    pub deadline_at: Option<DateTime<Utc>>,
    pub period_start: Option<DateTime<Utc>>,
    pub period_end: Option<DateTime<Utc>>,
    pub status: GoalStatus,
    pub priority: GoalPriority,
    pub goal_cycle: GoalLifeCycle,
    pub target_value: Option<i32>,
    pub current_value: Option<i32>,
    pub unit: Option<String>,
    pub parent_goal_id: Option<i32>,
}

#[derive(Debug, QueryableByName, Serialize)]
pub struct RoutinePartUsageRow {
    #[diesel(sql_type = Timestamptz)]
    pub month: DateTime<Utc>,

    #[diesel(sql_type = Text)]
    pub description: String,

    #[diesel(sql_type = BigInt)]
    pub total_minutes: i64,

    #[diesel(sql_type = BigInt)]
    pub item_count: i64,
}

#[derive(Debug, Serialize)]
pub struct RoutinePartGroupedRows {
    pub month: DateTime<Utc>,
    pub rows: Vec<RoutinePartUsageRow>,
}
