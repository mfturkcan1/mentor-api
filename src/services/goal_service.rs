use crate::db::AsyncPoolObject;
use crate::models::{Goals, NewGoal};
use crate::repositories::goal_repository::{delete_goal, get_goals, insert_goals};

pub async fn select_goals(conn: &mut AsyncPoolObject) -> Result<Vec<Goals>, diesel::result::Error> {
    get_goals(conn).await
}

pub async fn add_goals(
    conn: &mut AsyncPoolObject,
    new_goals: Vec<NewGoal>,
) -> Result<Vec<Goals>, diesel::result::Error> {
    insert_goals(conn, new_goals).await
}

pub async fn remove_goal(
    conn: &mut AsyncPoolObject,
    id: i32,
) -> Result<usize, diesel::result::Error> {
    delete_goal(conn, id).await
}
