//! Shared Postgres pool for integration-style tests under `feature = "ssr"`.
//!
//! Uses the same `DATABASE_URL` as local `cargo leptos watch` (via `.env`).
//! When the URL is unset or connect fails, callers should soft-skip.

use std::sync::Arc;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

static SHARED_POOL: OnceCell<Option<Arc<PgPool>>> = OnceCell::const_new();

/// Process-wide optional pool for tests. Safe to call from many tests.
pub async fn shared_pool() -> Option<&'static PgPool> {
    let slot = SHARED_POOL
        .get_or_init(|| async {
            dotenvy::dotenv().ok();
            let Ok(url) = std::env::var("DATABASE_URL") else {
                return None;
            };
            match PgPoolOptions::new().max_connections(3).connect(&url).await {
                Ok(pool) => Some(Arc::new(pool)),
                Err(err) => {
                    eprintln!("test shared_pool connect failed: {err}");
                    None
                }
            }
        })
        .await;

    slot.as_ref().map(|arc| arc.as_ref())
}
