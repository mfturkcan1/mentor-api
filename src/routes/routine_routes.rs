use crate::routes::{get_response_from_diesel_result, IdRequest};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{ Json, Router};
use mentor_api::services::{
    create_routine_with_parts, select_only_routines, select_routine_parts_by_id,
    select_routines, CreateRoutineRequest,
};

async fn create_routine(
    Json(payload): Json<CreateRoutineRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = create_routine_with_parts(payload);

    get_response_from_diesel_result(result)
}

async fn get_routines_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let routines = select_routines();
    get_response_from_diesel_result(routines)
}

async fn get_routines_only_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let routines = select_only_routines();
    get_response_from_diesel_result(routines)
}

async fn get_routine_parts_by_routine_id_endpoint(
    Query(routine_id): Query<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let routine_parts = select_routine_parts_by_id(routine_id.id);
    get_response_from_diesel_result(routine_parts)
}

pub fn init_routes() -> Router {
    Router::new()
        .route("/routine", get(get_routines_endpoint))
        .route("/routine", post(create_routine))
        .route("/routine/only", get(get_routines_only_endpoint))
        .route(
            "/routine/parts",
            get(get_routine_parts_by_routine_id_endpoint),
        )
}
