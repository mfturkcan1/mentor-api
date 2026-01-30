use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

mod goal_routes;
mod routine_routes;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Deserialize)]
pub struct IdRequest {
    pub id: i32,
}

pub fn get_response_from_diesel_result<T: Serialize>(
    t: Result<T, diesel::result::Error>,
) -> Result<impl IntoResponse, StatusCode> {
    match t {
        Ok(r) => Ok(Json(r).into_response()),
        Err(e) => {
            let errors_response = ErrorResponse {
                error: e.to_string(),
            };
            Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(errors_response)).into_response())
        }
    }
}

pub fn init_routes() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routine_routes::init_routes())
        .merge(goal_routes::init_routes())
        .layer(cors_layer)
}
