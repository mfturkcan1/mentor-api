use crate::models::{Goals, NewGoal};
use crate::schema::goals;
use chrono::Utc;
use diesel::QueryDsl;
use diesel::{ExpressionMethods, SelectableHelper};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

pub async fn insert_goals(
    conn: &mut AsyncPgConnection,
    new_goals: Vec<NewGoal>,
) -> Result<Vec<Goals>, diesel::result::Error> {
    diesel::insert_into(goals::table)
        .values(&new_goals)
        .returning(Goals::as_returning())
        .get_results(conn)
        .await
}

pub async fn get_goals(conn: &mut AsyncPgConnection) -> Result<Vec<Goals>, diesel::result::Error> {
    goals::table
        .filter(goals::delete_date.is_null())
        .select(Goals::as_select())
        .get_results(conn)
        .await
}

pub async fn delete_goal(
    conn: &mut AsyncPgConnection,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    diesel::update(goals::table)
        .filter(goals::id.eq(id))
        .set(goals::delete_date.eq(Utc::now()))
        .execute(conn)
        .await
}
