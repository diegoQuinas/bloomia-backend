use std::{env, sync::Arc};

use axum::{Router, routing::get};
use dotenvy::dotenv;
use sqlx::PgPool;

struct AppState {
    db: PgPool,
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt().with_env_filter("info").init();

    let db_url = env::var("DATABASE_URL")?;
    let db = PgPool::connect(&db_url).await?;
    tracing::info!("Connected to Postgres");
    let state = Arc::new(AppState { db });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
