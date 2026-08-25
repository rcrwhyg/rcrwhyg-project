use axum::Json;
use axum::extract::State;
use serde::Serialize;

use super::state::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub db: DbHealth,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DbHealth {
    Connected,
    Unset,
    Error,
}

/// `GET /health` — process liveness + optional DB probe.
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let db = match &state.db {
        None => DbHealth::Unset,
        Some(pool) => match sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(pool)
            .await
        {
            Ok(_) => DbHealth::Connected,
            Err(_) => DbHealth::Error,
        },
    };

    Json(HealthResponse { ok: true, db })
}
