mod routes;
pub mod schema;
mod server;

#[tokio::main]
async fn main() {
    server::init_server().await;
}
