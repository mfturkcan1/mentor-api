use deadpool::managed::{BuildError, Object, PoolError};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolError as ConnPoolError};
use std::env;
use dotenvy::dotenv;

pub type AsyncPool = deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type AsyncPoolObject = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;
pub type AsyncPoolError = PoolError<ConnPoolError>;

pub fn establish_connection_pool() -> Result<AsyncPool, BuildError> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    Pool::builder(config).build()
}

pub async fn get_from_pool(pool: AsyncPool) -> Result<AsyncPoolObject, AsyncPoolError> {
    pool.get().await
}
