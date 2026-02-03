pub mod db;
pub mod enums;
pub mod models;
pub mod repositories;
pub mod schema;
pub mod services;

use diesel::prelude::*;
use diesel_async::{AsyncConnection, AsyncPgConnection};
use dotenvy::dotenv;
use std::env;

pub async fn establish_async_connection() -> AsyncPgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url.as_str())
        .await
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
