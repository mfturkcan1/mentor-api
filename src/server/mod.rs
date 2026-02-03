use crate::routes;
use utoipa_swagger_ui::SwaggerUi;

pub async fn init_server() {
    let (app, doc) = routes::init_routes().split_for_parts();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();

    let app = app.merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", doc));

    axum::serve(listener, app).await.unwrap();
}
