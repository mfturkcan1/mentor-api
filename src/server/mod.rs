use crate::routes;

pub async fn init_server() {
    let app = routes::init_routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
