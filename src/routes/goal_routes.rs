use crate::routes::{get_response_from_diesel_result, IdRequest};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use axum::extract::Query;
use mentor_api::models::NewGoal;
use mentor_api::services::{add_goals, remove_goal, select_goals};

async fn get_goals_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let goals = select_goals();
    get_response_from_diesel_result(goals)
}

async fn insert_goals_endpoint(
    Json(payload): Json<Vec<NewGoal>>,
) -> Result<impl IntoResponse, StatusCode> {
    let goals = add_goals(payload);
    get_response_from_diesel_result(goals)
}

async fn delete_goal_endpoint(
    Query(id): Query<IdRequest>
) -> Result<impl IntoResponse, StatusCode>{
    let result = remove_goal(id.id);
    get_response_from_diesel_result(result)
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/goals", get(get_goals_endpoint))
        .route("/goals", post(insert_goals_endpoint))
        .route("/goals", delete(delete_goal_endpoint))
}
