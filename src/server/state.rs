use axum::extract::FromRef;
use leptos::config::LeptosOptions;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// Shared Axum + Leptos state.
#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub db: Option<PgPool>,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions) -> Self {
        let db = connect_optional_pool().await;
        Self { leptos_options, db }
    }

    pub fn provide_leptos_context(&self) {
        if let Some(pool) = self.db.clone() {
            leptos::prelude::provide_context(pool);
        }
    }
}

async fn connect_optional_pool() -> Option<PgPool> {
    let Ok(url) = std::env::var("DATABASE_URL") else {
        leptos::logging::log!("DATABASE_URL unset; continuing without Postgres");
        return None;
    };

    match PgPoolOptions::new().max_connections(5).connect(&url).await {
        Ok(pool) => {
            leptos::logging::log!("Postgres pool ready");
            Some(pool)
        }
        Err(err) => {
            leptos::logging::log!("Postgres connect failed ({err}); continuing without DB");
            None
        }
    }
}
