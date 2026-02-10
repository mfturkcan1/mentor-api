use crate::dto::todo_dto::TodoSelectDto;
use crate::routes::{AppState, get_response_from_diesel_result};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use mentor_api::db::get_from_pool;
use mentor_api::models::NewTodo;
use mentor_api::repositories::todo_repository::{get_todos, insert_todo};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[utoipa::path(tag = "Todos", post, path = "/todos", responses((status = OK, body = TodoSelectDto)))]
async fn insert_todo_endpoint(
    state: State<AppState>,
    Json(payload): Json<NewTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = get_from_pool(state.pool.clone()).await;
    let res = insert_todo(&mut conn, payload).await;
    get_response_from_diesel_result(res)
}

#[utoipa::path(tag = "Todos", get, path = "/todos", responses((status = OK, body = Vec<TodoSelectDto>)))]
async fn get_todos_endpoint(state: State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let mut conn = get_from_pool(state.pool.clone()).await;
    let results = get_todos(&mut conn).await;
    get_response_from_diesel_result(results)
}

pub fn init_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(insert_todo_endpoint))
        .routes(routes!(get_todos_endpoint))
}
