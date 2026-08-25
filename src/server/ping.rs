use leptos::prelude::*;

/// Demo server function for `<Suspense>` / Resource patterns.
#[server(ServerPing)]
pub async fn server_ping() -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;
    }
    Ok(String::from("pong · leptos server fn ok"))
}

/// Reports whether a Postgres pool was injected into Leptos context.
#[server(DbStatus)]
pub async fn db_status() -> Result<String, ServerFnError> {
    #[cfg(feature = "ssr")]
    {
        match use_context::<sqlx::PgPool>() {
            Some(pool) => {
                match sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&pool).await {
                    Ok(n) => Ok(format!("postgres ok (select {n})")),
                    Err(err) => Err(ServerFnError::new(format!("postgres query failed: {err}"))),
                }
            }
            None => Ok(String::from(
                "postgres unset (set DATABASE_URL to enable pool)",
            )),
        }
    }
    #[cfg(not(feature = "ssr"))]
    {
        Ok(String::from("client stub"))
    }
}
