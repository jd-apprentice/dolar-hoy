use crate::{get_env, load_env};
use sqlx::postgres::PgPool;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    load_env();

    let pool = PgPool::connect(&get_env("DATABASE_URL")).await?;

    Ok(pool)
}
