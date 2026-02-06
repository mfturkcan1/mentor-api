use axum::Json;
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use diesel::result::Error;
use mentor_api::db::{AsyncPool, establish_connection_pool};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

mod goal_routes;
mod routine_routes;

#[derive(OpenApi)]
#[openapi(info(title = "Mentor API", version = "1.0", description = "An mentor API"))]
pub struct ApiDoc;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct IdRequest {
    pub id: i32,
}

pub fn get_response_from_diesel_result<T: Serialize>(
    t: Result<T, Error>,
) -> Result<impl IntoResponse, StatusCode> {
    match t {
        Ok(r) => Ok(Json(r).into_response()),
        Err(e) => get_error_response(e),
    }
}

pub fn get_error_response(e: Error) -> Result<Response<Body>, StatusCode> {
    let errors_response = ErrorResponse {
        error: e.to_string(),
    };
    Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(errors_response)).into_response())
}

#[derive(Clone)]
pub struct AppState {
    pool: AsyncPool,
}

pub fn init_routes() -> OpenApiRouter {
    let pool =
        establish_connection_pool().unwrap_or_else(|_| panic!("Error creating database pool"));

    let state = AppState { pool };

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(goal_routes::init_routes())
        .merge(OpenApiRouter::from(routine_routes::init_routes()))
        .layer(cors_layer)
        .with_state(state)
}
