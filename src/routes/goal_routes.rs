use crate::routes::{
    AppState, IdRequest, get_error_response, get_error_response_connection,
    get_response_from_diesel_result,
};
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mentor_api::db::get_from_pool;
use mentor_api::models::{Goals, NewGoal};
use mentor_api::services::goal_service::{add_goals, remove_goal, select_goals};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(tag = "Goals", get, path = "/goals", responses((status = OK, body = Vec<Goals>)))]
async fn get_goals_endpoint(state: State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = get_from_pool(state.pool.clone()).await;

    let goals = select_goals(&mut conn).await;
    get_response_from_diesel_result(goals)
}

#[utoipa::path(tag = "Goals", post, path = "/goals", responses((status = OK, body = Vec<Goals>)))]
async fn insert_goals_endpoint(
    state: State<AppState>,
    Json(payload): Json<Vec<NewGoal>>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = get_from_pool(state.pool.clone()).await;

    let goals = add_goals(&mut conn, payload).await;
    get_response_from_diesel_result(goals)
}

#[utoipa::path(tag = "Goals", delete, path = "/goals", responses((status = OK, body = usize)))]
async fn delete_goal_endpoint(
    state: State<AppState>,
    Query(id): Query<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = get_from_pool(state.pool.clone()).await;

    let result = remove_goal(&mut conn, id.id).await;
    get_response_from_diesel_result(result)
}

pub fn init_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_goals_endpoint))
        .routes(routes!(insert_goals_endpoint))
        .routes(routes!(delete_goal_endpoint))
}
