use crate::routes::{IdRequest, get_response_from_diesel_result};
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mentor_api::models::{Goals, NewGoal};
use mentor_api::services::{add_goals, remove_goal, select_goals};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(tag= "Goals", get, path = "/goals", responses((status = OK, body = Vec<Goals>)))]
async fn get_goals_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let goals = select_goals().await;
    get_response_from_diesel_result(goals)
}

#[utoipa::path(tag= "Goals", post, path = "/goals", responses((status = OK, body = Vec<Goals>)))]
async fn insert_goals_endpoint(
    Json(payload): Json<Vec<NewGoal>>,
) -> Result<impl IntoResponse, StatusCode> {
    let goals = add_goals(payload).await;
    get_response_from_diesel_result(goals)
}

#[utoipa::path(tag= "Goals", delete, path = "/goals", responses((status = OK, body = usize)))]
async fn delete_goal_endpoint(
    Query(id): Query<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = remove_goal(id.id).await;
    get_response_from_diesel_result(result)
}

pub fn init_routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(get_goals_endpoint))
        .routes(routes!(insert_goals_endpoint))
        .routes(routes!(delete_goal_endpoint))
}
