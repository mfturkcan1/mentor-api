use crate::schema::{routine_parts, routines};
use chrono::{DateTime, Utc};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize)]
#[diesel(table_name = routines)]
pub struct Routine {
    pub id: i32,
    pub title: String,
    pub create_date: DateTime<Utc>,
}

#[derive(Queryable, Insertable, Debug, PartialEq)]
#[diesel(table_name = routines)]
pub struct NewRoutine<'a> {
    pub title: &'a str,
    pub create_date: DateTime<Utc>,
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
