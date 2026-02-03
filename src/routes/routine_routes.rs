use crate::routes::{AppState, IdRequest, get_response_from_diesel_result};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use mentor_api::services::{
    CreateRoutinePartRequest, CreateRoutineRequest, create_routine_parts,
    create_routine_with_parts, remove_routine, remove_routine_part, select_category_names,
    select_only_routines, select_routine_by_id, select_routine_parts_by_id,
    select_routine_parts_group_by_result, select_routines,
};

async fn create_routine(
    Json(payload): Json<CreateRoutineRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = create_routine_with_parts(payload);

    get_response_from_diesel_result(result)
}

async fn append_routine_parts(
    Query(routine_id): Query<IdRequest>,
    Json(payload): Json<Vec<CreateRoutinePartRequest>>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = create_routine_parts(payload, routine_id.id);
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

async fn get_routine_by_id(
    Path(IdRequest { id }): Path<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let routine_response = select_routine_by_id(id);

    get_response_from_diesel_result(routine_response)
}

async fn delete_routine_by_id_endpoint(
    Path(IdRequest { id }): Path<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let routine_response = remove_routine(id);
    get_response_from_diesel_result(routine_response)
}

async fn delete_routine_part_by_id_endpoint(
    Query(id): Query<IdRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let response = remove_routine_part(id.id);
    get_response_from_diesel_result(response)
}

async fn get_category_names_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let names = select_category_names();
    get_response_from_diesel_result(names)
}

async fn get_routine_parts_groups_endpoint() -> Result<impl IntoResponse, StatusCode> {
    let res = select_routine_parts_group_by_result();
    get_response_from_diesel_result(res)
}

pub fn init_routes() -> Router<AppState> {
    Router::new()
        .route("/routine", get(get_routines_endpoint))
        .route("/routine/{id}", get(get_routine_by_id))
        .route("/routine/{id}", delete(delete_routine_by_id_endpoint))
        .route("/routine", post(create_routine))
        .route("/routine/only", get(get_routines_only_endpoint))
        .route("/routine/parts/append", post(append_routine_parts))
        .route(
            "/routine/parts/group",
            get(get_routine_parts_groups_endpoint),
        )
        .route("/routine/parts", delete(delete_routine_part_by_id_endpoint))
        .route(
            "/routine/parts",
            get(get_routine_parts_by_routine_id_endpoint),
        )
        .route("/category/names", get(get_category_names_endpoint))
}
